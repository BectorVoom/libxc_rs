//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3149/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3149(t24770: f64, t73: f64, t1214: f64, t12809: f64, t12855: f64, t12910: f64, t17459: f64, t17605: f64, t17709: f64, t20800: f64, t21028: f64, t21119: f64, t21157: f64, t24704: f64, t24729: f64, t3625: f64, t3626: f64, t3629: f64, t3720: f64, t44738: f64, t5407: f64, t56727: f64, t56740: f64, t56742: f64, t71275: f64, t82293: f64) -> (f64, f64) {
    let t82725 = t24770 * t73;
    let t82730 = -t56727 + 0.45732285992607719436e-2_f64 * t71275 * t5407 + 0.22866142996303859718e-2_f64 * t17605 * t21157 - 0.12862205435420921092e-2_f64 * t12855 * t3720 * t24704 * t21119 + 0.64311027177104605458e-3_f64 * t12809 * t3720 * t24704 * t21028 + 0.12862205435420921092e-2_f64 * t12910 * t3720 * t24704 * t17459 - 0.85748036236139473947e-3_f64 * t17709 * t3626 * t82293 * t44738 - t56740 - 0.95275595817932748825e-4_f64 * t56742 - 0.12862205435420921092e-2_f64 * t12855 * t3720 * t20800 * t24729 * t1214 - 0.14291339372689912324e-3_f64 * t3625 * t3626 * t82725 * t3629;
    (t82725, t82730)
}
