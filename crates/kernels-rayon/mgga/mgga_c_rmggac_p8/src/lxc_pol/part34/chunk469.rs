//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 469/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk469(t11666: f64, t2564: f64, t11670: f64, t11654: f64, t507: f64, t880: f64) -> (f64, f64, f64, f64) {
    let t12111 = t2564 * t11666;
    let t12117 = t2564 * t11670;
    let t12140 = t2564 * t11654;
    let t12200 = t507 * t880;
    (t12111, t12117, t12140, t12200)
}
