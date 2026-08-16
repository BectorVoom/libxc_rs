//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 791/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk791<F: Float>(t1066: F, t5633: F, t2118: F, t2989: F, t2968: F, t5952: F, t5728: F, t6012: F) -> (F, F, F, F) {
    let t7796 = t5633 * t1066;
    let t7824 = t2118 * t2989;
    let t7831 = t5952 * t2968;
    let t7832 = t5728 * t6012;
    (t7796, t7824, t7831, t7832)
}
