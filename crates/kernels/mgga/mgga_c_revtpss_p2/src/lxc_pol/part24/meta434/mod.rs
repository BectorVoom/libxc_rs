//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta434 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1385;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1386;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta434<F: Float>(t1429: F, t39501: F, t544: F, t9989: F, t555: F, t4003: F, t1433: F, t39545: F, t546: F, t685: F, t39552: F, t557: F, t1408: F, t820: F, t9948: F, t240: F, t9991: F, t549: F, t72: F, t2237: F, t2482: F, t1369: F, t9726: F, t9801: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t46412, t46475, t46476, t46478, t46515, t46518) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1385::<F>(t1429, t39501, t544, t9989, t555, t4003, t1433, t39545, t546, t685, t39552, t557);
        let (t46595, t46609, t46627, t46644, t46651, t46670) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1386::<F>(t1408, t820, t9948, t240, t9991, t549, t72, t2237, t2482, t1369, t9726, t546, t9801);
    (t46412, t46475, t46476, t46478, t46515, t46518, t46595, t46609, t46627, t46644, t46651, t46670)
}
