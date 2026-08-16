//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta618 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1936;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1937;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta618(t16215: f64, t221: f64, t91194: f64, t6604: f64, t80893: f64, t1361: f64, t6925: f64, t6976: f64, t22828: f64, t26243: f64, t26271: f64, t80779: f64, t22844: f64, t7708: f64, t16391: f64, t26309: f64, t5259: f64, t80820: f64, t16265: f64, t22833: f64, t5293: f64, t80816: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91196, t91200, t91204, t91206) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1936(t16215, t221, t91194, t6604, t80893, t1361, t6925, t6976, t22828, t26243, t26271, t80779);
        let (t91210, t91212, t91214, t91216, t91218) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1937(t22844, t6976, t22828, t7708, t16391, t26309, t5259, t80820, t16265, t22833, t5293, t80816);
    (t91196, t91200, t91204, t91206, t91210, t91212, t91214, t91216, t91218)
}
