//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 616/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk616<F: Float>(t1145: F, t2893: F, t1144: F, t1149: F, t1158: F, t1172: F, t2821: F, t2825: F, t2829: F, t2831: F, t2834: F, t2838: F, t2842: F, t2847: F, t2853: F, t2859: F, t2862: F, t2868: F, t2869: F, t2875: F, t2877: F, t2881: F, t2890: F) -> (F, F) {
    let t2894 = t1145 * t2893;
    let t2897 = -F::new(16.0) / F::new(9.0) * t2821 * t2825 + F::new(16.0) / F::new(9.0) * t2829 * t2831 - F::new(16.0) / F::new(3.0) * t2834 * t2825 + F::new(16.0) / F::new(3.0) * t2838 * t2831 + F::new(44.0) / F::new(27.0) * t1172 * t2842 + F::new(44.0) / F::new(27.0) * t1158 * t2842 - F::new(32.0) / F::new(81.0) * t2847 * t2853 - F::new(16.0) / F::new(27.0) * t1158 * t2859 - F::new(32.0) / F::new(81.0) * t2862 * t2853 - F::new(16.0) / F::new(27.0) * t1172 * t2859 + F::new(15.0) * t2868 * t1145 * t2869 + F::new(21.0) * t2875 * t2877 + F::new(3.0) * t2881 * t2877 + F::new(3.0) * t1144 * t2890 - F::new(3.0) * t1149 * t2894;
    (t2894, t2897)
}
