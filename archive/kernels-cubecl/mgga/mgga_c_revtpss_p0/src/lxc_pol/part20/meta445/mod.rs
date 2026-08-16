//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta445 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1702;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1703;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta445<F: Float>(t10008: F, t213: F, t10153: F, t2435: F, t2439: F, t3895: F, t4078: F, t39552: F, t562: F, t560: F, t9655: F, t225: F, t4077: F, t3896: F, t39515: F, t3900: F, t9292: F, t1419: F, t9646: F, t9648: F, t10147: F, t1357: F, t689: F, t1362: F, t1363: F, t39497: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t46350, t46353, t46356, t46359, t46362) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1702::<F>(t10008, t213, t10153, t2435, t2439, t3895, t4078, t39552, t562, t560, t9655, t225);
        let (t46363, t46368, t46369, t46378, t46381, t46385) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1703::<F>(t4077, t3896, t39515, t3900, t9292, t1419, t9646, t9648, t10147, t1357, t689, t1362, t1363, t39497);
    (t46350, t46353, t46356, t46359, t46362, t46363, t46368, t46369, t46378, t46381, t46385)
}
