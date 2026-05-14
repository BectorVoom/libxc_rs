//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1182/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1182<F: Float>(t1880: F, t1913: F, t1941: F, t19824: F, t3032: F, t3827: F, t3844: F, t3847: F, t3849: F, t3851: F, t3853: F, t51: F, t54: F, t584: F, t60: F, t6004: F, t63: F, t66: F, t69: F, t7984: F, t9915: F, t9934: F, t9937: F, t9942: F, t9945: F) -> (F,) {
    let t27222 = -19.0 / 412876800.0 * t19824 * t3827 * t1880 + t6004 * t3844 * t1880 / 412876800.0 + 10.0 / 3.0 * t3847 * t1880 - 2.0 / 3.0 * t3849 * t1880 - 7.0 / 8.0 * t3851 * t1880 + t3853 * t1880 / 8.0 + t9934 * t1913 / 6.0 + t9937 * t1913 / 8.0 - t3032 * t7984 / 24.0 - t9942 * t1913 / 48.0 - t60 * t9915 * t584 / 5760.0 + t63 * t9915 * t584 / 129024.0 - t66 * t9915 * t584 / 3440640.0 + t69 * t9915 * t584 / 0.10616832e9 - t1941 * t9915 * t584 / 0.37158912e10 + t51 * t9915 * t584 / 3.0 - t54 * t9915 * t584 / 24.0 - t9945 * t1913 / 80.0;
    (t27222,)
}
