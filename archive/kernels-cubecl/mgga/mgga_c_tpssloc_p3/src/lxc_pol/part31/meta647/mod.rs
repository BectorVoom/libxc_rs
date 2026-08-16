//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta647 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1920;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta647<F: Float>(t22893: F, t23164: F, t28345: F, t23153: F, t5544: F, t6552: F, t6637: F, t16662: F, t6638: F, t28329: F, t16927: F, t87052: F, t87529: F, t23185: F, t28426: F, t81914: F, t25248: F, t776: F, t87642: F, t98336: F, t28334: F, t6547: F, t28322: F, t6579: F, t16762: F, t1888: F, t6646: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t98345, t98349, t98353, t98356, t98359) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1920::<F>(t22893, t23164, t28345, t23153, t5544, t6552, t6637, t16662, t6638, t28329, t16927, t87052, t87529);
        let (t98363, t98367, t98374, t98380, t98384) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1921::<F>(t23185, t28426, t81914, t25248, t776, t87642, t98336, t28334, t6547, t28322, t6579, t16762, t1888, t6646);
    (t98345, t98349, t98353, t98356, t98359, t98363, t98367, t98374, t98380, t98384)
}
