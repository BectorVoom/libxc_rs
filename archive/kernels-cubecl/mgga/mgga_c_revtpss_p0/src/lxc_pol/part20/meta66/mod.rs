//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta66 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk435;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk436;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta66<F: Float>(t1357: F, t1358: F, t689: F, t556: F, t786: F, t561: F, t72: F, t686: F, t535: F, t795: F, t159: F, t540: F, t216: F, t124: F, t1353: F, t800: F, t546: F, t550: F, t808: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1359, t1361, t1362, t1363, t1364, t1366, t1368, t1369) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk435::<F>(t1357, t1358, t689, t556, t786, t561, t72, t686, t535, t795, t159, t540);
        let (t1370, t1371, t1372, t1376) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk436::<F>(t1369, t216, t124, t1353, t800, t546, t550, t808);
    (t1359, t1361, t1362, t1363, t1364, t1366, t1368, t1369, t1370, t1371, t1372, t1376)
}
