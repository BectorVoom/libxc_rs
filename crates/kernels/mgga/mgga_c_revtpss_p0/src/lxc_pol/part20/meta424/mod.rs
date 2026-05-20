//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta424 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1591;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1592;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta424<F: Float>(t406: F, t43822: F, t12254: F, t141: F, t43835: F, t1145: F, t43843: F, t1139: F, t43908: F, t3407: F, t43825: F, t43886: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t43899: F, t43902: F, t43905: F, t43806: F, t43856: F, t43936: F, t1179: F, t1188: F, t1196: F, t3515: F, t3520: F, t3523: F, t3794: F, t12555: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t43947, t43950, t43953, t43955, t43957, t43959) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1591::<F>(t406, t43822, t12254, t141, t43835, t1145, t43843, t1139, t43908, t3407, t43825, t43886, t43888, t43890, t43892, t43894, t43896, t43899, t43902, t43905);
        let (t43961, t43965, t43966, t43970, t43971, t43977) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1592::<F>(t43806, t43856, t43936, t43959, t1179, t1188, t1196, t3515, t3520, t3523, t3794, t12555);
    (t43947, t43950, t43953, t43955, t43957, t43961, t43965, t43966, t43970, t43971, t43977)
}
