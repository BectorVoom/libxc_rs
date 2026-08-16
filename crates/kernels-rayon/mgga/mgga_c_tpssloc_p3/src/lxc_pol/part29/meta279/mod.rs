//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta279 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1288;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1289;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1290;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta279(t7320: f64, t8034: f64, t1734: f64, t68: f64, t475: f64, t7328: f64, t1730: f64, t2140: f64, t1742: f64, t2139: f64, t471: f64, t1726: f64, t1737: f64, t1748: f64, t2134: f64, t2136: f64, t467: f64, t488: f64, t7309: f64, t7310: f64, t7315: f64, t7326: f64, t7339: f64, t7343: f64, t7345: f64, t8020: f64, t8028: f64, t8031: f64, t466: f64, t1760: f64, t2154: f64, t3598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t8035, t8038, t8039, t8040) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1288(t7320, t8034, t1734, t68, t475, t7328);
        let (t8043, t8048, t8049, t8054) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1289(t1730, t2140, t1742, t2139, t471, t1726, t1737, t1748, t2134, t2136, t467, t488, t7309, t7310, t7315, t7326, t7339, t7343, t7345, t8020, t8028, t8031, t8035, t8040);
        let (t8055, t8061) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1290(t466, t8054, t1760, t2154, t3598);
    (t8035, t8038, t8039, t8040, t8043, t8048, t8049, t8054, t8055, t8061)
}
