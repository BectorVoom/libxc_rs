//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta650 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2064;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta650(t23665: f64, t25545: f64, t25503: f64, t10216: f64, t381: f64, t10474: f64, t82514: f64, t25483: f64, t23384: f64, t25456: f64, t362: f64, t4657: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t89158, t89175, t89176, t89204, t89210, t89224, t89235) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2064(t23665, t25545, t25503, t10216, t381, t10474, t82514, t25483, t23384, t25456, t362, t4657);
    (t89158, t89175, t89176, t89204, t89210, t89224, t89235)
}
