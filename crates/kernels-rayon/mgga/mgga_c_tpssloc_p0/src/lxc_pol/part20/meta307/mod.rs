//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta307 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1554;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1555;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta307(t1100: f64, t11258: f64, t1107: f64, t410: f64, t417: f64, t11244: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t11150: f64, t11156: f64, t11165: f64, t11174: f64, t11230: f64, t11233: f64, t11245: f64, t11228: f64, t1118: f64, t1099: f64, t1097: f64, t3311: f64, t409: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11259, t11261, t11265, t11266, t11268) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1554(t1100, t11258, t1107, t410, t417, t11244, t11137, t11139, t11141, t11143, t11150, t11156, t11165, t11174, t11230, t11233, t11245);
        let (t11269, t11270, t11272, t11274, t11275) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1555(t11228, t11268, t1118, t1099, t1097, t3311, t409);
    (t11259, t11261, t11265, t11266, t11269, t11270, t11272, t11274, t11275)
}
