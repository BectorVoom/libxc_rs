//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 964/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk964<F: Float>(t8840: F, t8852: F, t810: F, t788: F, t3363: F, t820: F, t1351: F, t2272: F, t3313: F, t787: F, t811: F, t2253: F, t2268: F, t2273: F, t2276: F, t2292: F, t2312: F, t3366: F, t830: F, t8736: F, t8738: F, t8741: F, t8815: F, t8818: F, t8821: F, t8824: F, t8828: F) -> (F, F, F, F, F, F, F, F) {
    let t8853 = t8840 + t8852;
    let t8854 = t8853 * t810;
    let t8856 = F::cast_from(1.0_f64) * t788 * t8854;
    let t8857 = t3363 * t820;
    let t8862 = t1351 * t2272;
    let t8865 = t3313 * t787;
    let t8867 = F::cast_from(2.0_f64) * t8865 * t811;
    let t8868 = t8736 - t8738 + t8741 + F::cast_from(0.35089341735807877242e1_f64) * t2312 * t8815 + F::cast_from(6.0_f64) * t2273 * t8818 - F::cast_from(2.0_f64) * t8821 * t2253 - F::cast_from(0.11696447245269292414e1_f64) * t8824 * t2292 - t8828 - t8856 + F::cast_from(2.0_f64) * t8857 * t830 + F::cast_from(1.0_f64) * t3366 * t2268 + F::cast_from(0.32163958997385070134e2_f64) * t8862 * t2276 - t8867;
    (t8853, t8854, t8856, t8857, t8862, t8865, t8867, t8868)
}
