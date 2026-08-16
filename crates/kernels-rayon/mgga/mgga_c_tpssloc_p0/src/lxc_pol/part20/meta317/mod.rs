//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta317 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1578;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1579;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1580;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta317(t11529: f64, t1179: f64, t1174: f64, t3431: f64, t3460: f64, t3456: f64, t135: f64, t3439: f64, t3442: f64, t11499: f64, t11505: f64, t11510: f64, t11514: f64, t11518: f64, t11522: f64, t11526: f64, t3247: f64, t405: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11530, t11531, t11533, t11534, t11536, t11537, t11539) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1578(t11529, t1179, t1174, t3431, t3460, t3456, t135, t3439);
        let (t11540, t11541, t11543) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1579(t11539, t3442, t1174, t11499, t11505, t11510, t11514, t11518, t11522, t11526, t11531, t11534, t11537);
        let t11545 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1580(t3247, t405);
    (t11530, t11531, t11533, t11534, t11536, t11537, t11539, t11540, t11541, t11543, t11545)
}
