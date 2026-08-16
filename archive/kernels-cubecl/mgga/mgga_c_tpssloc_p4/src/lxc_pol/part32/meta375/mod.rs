//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta375 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1429;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1430;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1431;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta375<F: Float>(t12189: F, t1804: F, t5194: F, t782: F, t5198: F, t3732: F, t67: F, t792: F, t1799: F, t212: F, t1307: F, t686: F, t12214: F, t131: F, t205: F, t3726: F, t5206: F, t12199: F, t5202: F, t118: F, t5187: F, t794: F, t3739: F, t12225: F, t2586: F, t1338: F, t5318: F, t3866: F, t5310: F, t3799: F, t5289: F, t2371: F, t5154: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16078, t16083, t16094, t16095, t16097) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1429::<F>(t12189, t1804, t5194, t782, t5198, t3732, t67, t792, t1799, t212, t1307, t686);
        let (t16099, t16101, t16106, t16108, t16111) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1430::<F>(t16094, t16097, t12214, t131, t205, t3726, t5206, t12199, t5202, t118, t5187, t794);
        let (t16113, t16119, t16132, t16147, t16159, t16164) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1431::<F>(t16111, t3739, t12225, t16095, t2586, t1338, t5318, t3866, t5310, t3799, t5289, t2371, t5154);
    (t16078, t16083, t16099, t16101, t16106, t16108, t16113, t16119, t16132, t16147, t16159, t16164)
}
