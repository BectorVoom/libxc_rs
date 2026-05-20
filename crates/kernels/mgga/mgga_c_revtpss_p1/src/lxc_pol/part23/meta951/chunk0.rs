//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3149/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3149<F: Float>(t24770: F, t73: F, t1214: F, t12809: F, t12855: F, t12910: F, t17459: F, t17605: F, t17709: F, t20800: F, t21028: F, t21119: F, t21157: F, t24704: F, t24729: F, t3625: F, t3626: F, t3629: F, t3720: F, t44738: F, t5407: F, t56727: F, t56740: F, t56742: F, t71275: F, t82293: F) -> (F, F) {
    let t82725 = t24770 * t73;
    let t82730 = -t56727 + F::cast_from(0.45732285992607719436e-2_f64) * t71275 * t5407 + F::cast_from(0.22866142996303859718e-2_f64) * t17605 * t21157 - F::cast_from(0.12862205435420921092e-2_f64) * t12855 * t3720 * t24704 * t21119 + F::cast_from(0.64311027177104605458e-3_f64) * t12809 * t3720 * t24704 * t21028 + F::cast_from(0.12862205435420921092e-2_f64) * t12910 * t3720 * t24704 * t17459 - F::cast_from(0.85748036236139473947e-3_f64) * t17709 * t3626 * t82293 * t44738 - t56740 - F::cast_from(0.95275595817932748825e-4_f64) * t56742 - F::cast_from(0.12862205435420921092e-2_f64) * t12855 * t3720 * t20800 * t24729 * t1214 - F::cast_from(0.14291339372689912324e-3_f64) * t3625 * t3626 * t82725 * t3629;
    (t82725, t82730)
}
