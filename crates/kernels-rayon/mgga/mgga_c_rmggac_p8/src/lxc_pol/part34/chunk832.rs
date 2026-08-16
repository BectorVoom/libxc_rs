//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 832/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk832(t15252: f64, t2144: f64, t333: f64, t3351: f64, t7231: f64, t498: f64, t8946: f64, t3352: f64, t8947: f64, t15128: f64, t321: f64, t262: f64) -> (f64, f64, f64, f64, f64) {
    let t74948 = t3351 * t7231 * t2144 * t15252 * t333;
    let t74953 = t3351 * t7231 * t2144 * t8946 * t498;
    let t74957 = t3351 * t3352 * t2144 * t8947;
    let t74959 = t15128 * t321;
    let t74960 = t262 * t74959;
    (t74948, t74953, t74957, t74959, t74960)
}
