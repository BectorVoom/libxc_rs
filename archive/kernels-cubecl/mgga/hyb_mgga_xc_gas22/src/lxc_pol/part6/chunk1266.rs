//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1266/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1266<F: Float>(t1880: F, t1913: F, t1941: F, t19824: F, t3032: F, t3827: F, t3844: F, t3847: F, t3849: F, t3851: F, t3853: F, t51: F, t54: F, t584: F, t60: F, t6004: F, t63: F, t66: F, t69: F, t7984: F, t9915: F, t9934: F, t9937: F, t9942: F, t9945: F) -> F {
    let t27222 = -F::cast_from(19.0_f64) / F::cast_from(412876800.0_f64) * t19824 * t3827 * t1880 + t6004 * t3844 * t1880 / F::cast_from(412876800.0_f64) + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t3847 * t1880 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3849 * t1880 - F::cast_from(7.0_f64) / F::cast_from(8.0_f64) * t3851 * t1880 + t3853 * t1880 / F::cast_from(8.0_f64) + t9934 * t1913 / F::cast_from(6.0_f64) + t9937 * t1913 / F::cast_from(8.0_f64) - t3032 * t7984 / F::cast_from(24.0_f64) - t9942 * t1913 / F::cast_from(48.0_f64) - t60 * t9915 * t584 / F::cast_from(5760.0_f64) + t63 * t9915 * t584 / F::cast_from(129024.0_f64) - t66 * t9915 * t584 / F::cast_from(3440640.0_f64) + t69 * t9915 * t584 / F::cast_from(0.10616832e9_f64) - t1941 * t9915 * t584 / F::cast_from(0.37158912e10_f64) + t51 * t9915 * t584 / F::cast_from(3.0_f64) - t54 * t9915 * t584 / F::cast_from(24.0_f64) - t9945 * t1913 / F::cast_from(80.0_f64);
    t27222
}
