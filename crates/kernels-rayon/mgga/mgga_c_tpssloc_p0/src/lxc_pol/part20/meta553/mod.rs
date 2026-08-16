//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta553 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2099;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2100;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta553(t2635: f64, t41424: f64, t2639: f64, t9663: f64, t13258: f64, t9634: f64, t9629: f64, t6589: f64, t67: f64, t246: f64, t232: f64, t9458: f64, t10046: f64, t814: f64, t225: f64, t9520: f64, t10647: f64, t892: f64, t2784: f64, t2841: f64, t22715: f64, t268: f64, t271: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41425, t41427, t41435, t41437, t41466, t41467, t41468) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2099(t2635, t41424, t2639, t9663, t13258, t9634, t9629, t6589, t67, t246, t232, t9458);
        let (t41520, t41554, t41618, t41623, t41654) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2100(t10046, t814, t225, t9520, t10647, t892, t2784, t2841, t22715, t268, t271);
    (t41425, t41427, t41435, t41437, t41466, t41467, t41468, t41520, t41554, t41618, t41623, t41654)
}
