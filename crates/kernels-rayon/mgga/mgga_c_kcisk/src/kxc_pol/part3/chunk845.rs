//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 845/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk845(t1192: f64, t12900: f64, t3634: f64, t3672: f64, t1167: f64, t3676: f64, t3680: f64, t317: f64, t3675: f64, t305: f64, t1190: f64, t3640: f64) -> (f64, f64, f64, f64, f64) {
    let t12902 = 3.0_f64 * t12900 * t1192;
    let t12904 = 3.0_f64 * t3634 * t3672;
    let t12905 = t1167 * t3676;
    let t12907 = 0.48245472966453314466e2_f64 * t12905 * t3680;
    let t12909 = 1.0_f64 / t3675 / t317;
    let t12910 = t305 * t12909;
    let t12911 = t3640 * t1190;
    (t12902, t12904, t12907, t12910, t12911)
}
