//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta673 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2229;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta673(t4509: f64, t5842: f64, t17686: f64, t42841: f64, t17783: f64, t2960: f64, t13779: f64, t17167: f64, t2986: f64, t17171: f64, t13784: f64, t17157: f64, t10190: f64, t17817: f64, t17769: f64, t10224: f64, t5824: f64, t973: f64, t13822: f64, t17752: f64, t17757: f64, t17772: f64, t2970: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61365, t61375, t61383, t61387, t61391, t61394) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2229(t4509, t5842, t17686, t42841, t17783, t2960, t13779, t17167, t2986, t17171, t13784, t17157);
        let (t61397, t61405, t61408, t61422, t61427, t61447) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2230(t10190, t17817, t2986, t17769, t2960, t10224, t5824, t973, t13822, t17752, t17757, t17772, t2970);
    (t61365, t61375, t61383, t61387, t61391, t61394, t61397, t61405, t61408, t61422, t61427, t61447)
}
