//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 803/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk803(t14125: f64, t68844: f64, t74376: f64, t68871: f64, t9146: f64, t3351: f64, t3352: f64, t875: f64, t8963: f64, t1971: f64, t7262: f64, t8937: f64) -> (f64, f64, f64, f64) {
    let t74439 = t68844 * t14125 * t74376;
    let t74442 = t68871 * t14125 * t9146;
    let t74446 = t3351 * t3352 * t875 * t8963;
    let t74450 = t3351 * t1971 * t7262 * t8937;
    (t74439, t74442, t74446, t74450)
}
