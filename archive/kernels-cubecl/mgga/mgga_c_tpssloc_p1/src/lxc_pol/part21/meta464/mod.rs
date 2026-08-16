//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta464 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2035;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2036;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta464<F: Float>(t221: F, t3734: F, t5196: F, t3726: F, t5206: F, t12199: F, t5202: F, t118: F, t5187: F, t794: F, t3739: F, t16018: F, t210: F, t214: F, t12225: F, t16095: F, t2586: F, t12236: F, t1315: F, t16083: F, t16086: F, t16090: F, t16099: F, t16101: F, t5195: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t16103, t16106, t16108, t16111, t16113, t16115) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2035::<F>(t221, t3734, t5196, t3726, t5206, t12199, t5202, t118, t5187, t794, t3739, t16018, t210, t214);
        let (t16118, t16119, t16121) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2036::<F>(t12225, t16095, t2586, t12236, t1315, t16083, t16086, t16090, t16099, t16101, t16103, t16106, t16108, t16113, t16115, t5195);
    (t16103, t16106, t16108, t16111, t16113, t16115, t16118, t16119, t16121)
}
