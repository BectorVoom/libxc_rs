//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1667;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1668;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1669;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta411<F: Float>(t28: F, t12072: F, t1649: F, t2: F, t3672: F, t1081: F, t584: F, t16: F, t3231: F, t3673: F, t5142: F, t5145: F, t517: F, zeta_threshold: F, t157: F, t15951: F, t182: F, t1787: F, t2516: F, t17: F, t12097: F, t12100: F, t12111: F, t12120: F, t184: F, t2663: F, t5157: F, t12103: F, t12105: F, t12109: F, t12114: F, t12116: F, t12118: F, t12123: F, t12477: F, t1799: F, t3719: F, t3918: F, t5122: F, t9797: F, t9820: F, t9824: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t15956, t15966) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1667::<F>(t28, t12072, t1649, t2, t3672, t1081, t584, t16, t3231, t3673, t5142, t5145, t517, zeta_threshold);
        let (t15970, t15972, t15973, t15974, t15975, t15976, t15977) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1668::<F>(t157, t15951, t15966, t182, t1787, t2516, t17, t12097, t12100, t12111, t12120, t184);
        let (t15978, t15980, t15981) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1669::<F>(t15977, t17, t2663, t5157, t12103, t12105, t12109, t12114, t12116, t12118, t12123, t12477, t15970, t15972, t15973, t15974, t15975, t15976, t1799, t3719, t3918, t5122, t9797, t9820, t9824);
    (t15956, t15970, t15972, t15973, t15974, t15975, t15976, t15978, t15980, t15981)
}
