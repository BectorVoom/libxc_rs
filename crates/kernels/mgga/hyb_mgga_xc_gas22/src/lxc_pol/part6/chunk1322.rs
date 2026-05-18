//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1322/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1322<F: Float>(t2213: F, t238: F, t4131: F, t3309: F, t242: F, t2195: F, t4121: F, t6585: F, t20625: F, t20689: F, t20694: F, t20703: F, t20706: F, t28850: F, t28853: F, t28856: F, t28859: F) -> (F, F, F, F) {
    let t28862 = t238 * t2213 * t4131;
    let t28864 = t3309 * t3309;
    let t28866 = t238 * t242 * t28864;
    let t28872 = t6585 * t4121 * t2195;
    let t28874 = F::new(0.16504875e0) * t28850 - F::new(0.60385e0) * t28853 + F::new(0.905775e0) * t28856 + F::new(0.40256666666666666667e0) * t28859 + F::new(0.27595e0) * t28862 + F::new(0.49671e0) * t28866 + t20689 + F::new(0.27595e0) * t20694 + t20625 - F::new(0.18786444444444444445e1) * t20703 + F::new(0.40256666666666666667e0) * t20706 + F::new(0.19419375e1) * t28872;
    (t28862, t28866, t28872, t28874)
}
