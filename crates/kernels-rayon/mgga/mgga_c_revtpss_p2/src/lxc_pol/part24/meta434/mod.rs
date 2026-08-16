//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta434 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1385;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1386;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta434(t1429: f64, t39501: f64, t544: f64, t9989: f64, t555: f64, t4003: f64, t1433: f64, t39545: f64, t546: f64, t685: f64, t39552: f64, t557: f64, t1408: f64, t820: f64, t9948: f64, t240: f64, t9991: f64, t549: f64, t72: f64, t2237: f64, t2482: f64, t1369: f64, t9726: f64, t9801: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46412, t46475, t46476, t46478, t46515, t46518) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1385(t1429, t39501, t544, t9989, t555, t4003, t1433, t39545, t546, t685, t39552, t557);
        let (t46595, t46609, t46627, t46644, t46651, t46670) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1386(t1408, t820, t9948, t240, t9991, t549, t72, t2237, t2482, t1369, t9726, t546, t9801);
    (t46412, t46475, t46476, t46478, t46515, t46518, t46595, t46609, t46627, t46644, t46651, t46670)
}
