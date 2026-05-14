//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1366/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1366<F: Float>(t12053: F, t2922: F, t1128: F, t2849: F, t4554: F, t12017: F, t4550: F, t1567: F, t1815: F, t33637: F, t10087: F, t10091: F, t1127: F, t1132: F, t12072: F, t15553: F, t15560: F, t15563: F, t15570: F, t15599: F, t15602: F, t2900: F, t2901: F, t2946: F, t33548: F, t33572: F, t4631: F, t7833: F, t7913: F, t7918: F, t8089: F) -> (F, F, F, F, F) {
    let t33685 = t2922 * t12053;
    let t33689 = t1128 * t4554 * t2849;
    let t33692 = t2922 * t12017;
    let t33698 = t1128 * t4550 * t2849;
    let t33701 = t1815 * t1567;
    let t33702 = t33701 * t33637;
    let t33723 = -0.12e-1 * t2900 * t33548 + 0.58666666666666666667e-1 * t1127 * t33685 - 0.72e-1 * t7918 * t33689 - 0.58666666666666666667e-1 * t1132 * t33692 - 0.768e-3 * t2946 * t33572 + 0.36e-1 * t8089 * t33698 + 70000.0 / 27.0 * t15553 * t33702 + 10000.0 / 81.0 * t15560 * t33702 + 40000.0 / 81.0 * t15563 * t33702 + 10000.0 / 27.0 * t15570 * t33702 + 50000.0 / 27.0 * t15599 * t33702 + 40000.0 / 9.0 * t15602 * t33702 - 0.176e-3 * t7913 * t4631 - 0.108e1 * t10087 * t12053 * t2901 - 0.48e0 * t10091 * t7833 * t12072;
    (t33685, t33689, t33692, t33698, t33723)
}
