//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1267/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1267(t3726: f64, t3770: f64, t12211: f64, t3766: f64, t1358: f64, t3774: f64, t1333: f64, t3862: f64, t10022: f64, t248: f64, t557: f64, t555: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12310 = t3726 * t3770;
    let t12317 = t12211 * t3766;
    let t12323 = t3774 * t1358;
    let t12325 = t1333 * t3862;
    let t12328 = t10022 * t557 * t248;
    let t12330 = 595.0_f64 / 10368.0_f64 * t555 * t12328;
    (t12310, t12317, t12323, t12325, t12328, t12330)
}
