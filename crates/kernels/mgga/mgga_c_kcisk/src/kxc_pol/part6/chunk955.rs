//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 955/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk955<F: Float>(t776: F, t12169: F, t28368: F, t10832: F, t28532: F, t41: F, t28800: F, t7568: F, t2442: F, t2620: F, t29275: F, t29282: F, t525: F, t642: F, t7567: F, t773: F, t8781: F, t8787: F, t9192: F) -> (F, F) {
    let t777 = t776 < -F::cast_from(0.66725e-1_f64);
    let t29890 = t12169 * t28368;
    let t29891 = t10832 * t29890;
    let t29894 = t28532 * t41;
    let t29910 = t7568 * t28800;
    let t29917 = piecewise3::<F>(t777, F::cast_from(0.0_f64), F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t525 * t29894 * t642 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t525 * t9192 * t2442 + F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t525 * t2620 * t8781 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t525 * t2620 * t8787 - F::cast_from(280.0_f64) / F::cast_from(243.0_f64) * t525 * t773 * t29275 + F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t7567 * t29910 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t525 * t773 * t29282);
    (t29891, t29917)
}
