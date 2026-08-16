//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 898/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk898(t1986: f64, t2398: f64, t7720: f64, t17695: f64, t511: f64, t7231: f64, t14258: f64, t11674: f64, t495: f64, t14230: f64, t14243: f64, t2067: f64) -> (f64, f64, f64) {
    let t76089 = t1986 * t2398;
    let t76090 = t7720 * t76089;
    let t76101 = t511 * t17695;
    let t76102 = t7231 * t76101;
    let t76103 = t14258 * t76102;
    let t76105 = t11674 * t495;
    let t76108 = t14230 * t14243 * t2067 * t76105;
    (t76090, t76103, t76108)
}
