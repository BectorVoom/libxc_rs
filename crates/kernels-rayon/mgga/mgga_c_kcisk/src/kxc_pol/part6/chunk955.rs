//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 955/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk955(t776: f64, t12169: f64, t28368: f64, t10832: f64, t28532: f64, t41: f64, t28800: f64, t7568: f64, t2442: f64, t2620: f64, t29275: f64, t29282: f64, t525: f64, t642: f64, t7567: f64, t773: f64, t8781: f64, t8787: f64, t9192: f64) -> (f64, f64) {
    let t777 = t776 < -0.66725e-1_f64;
    let t29890 = t12169 * t28368;
    let t29891 = t10832 * t29890;
    let t29894 = t28532 * t41;
    let t29910 = t7568 * t28800;
    let t29917 = piecewise3(t777, 0.0_f64, 10.0_f64 / 9.0_f64 * t525 * t29894 * t642 - 10.0_f64 / 9.0_f64 * t525 * t9192 * t2442 + 40.0_f64 / 27.0_f64 * t525 * t2620 * t8781 - 10.0_f64 / 9.0_f64 * t525 * t2620 * t8787 - 280.0_f64 / 243.0_f64 * t525 * t773 * t29275 + 40.0_f64 / 27.0_f64 * t7567 * t29910 - 10.0_f64 / 27.0_f64 * t525 * t773 * t29282);
    (t29891, t29917)
}
