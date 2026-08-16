//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3744/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3744(t459: f64, t71134: f64, t71148: f64, t71162: f64, t71176: f64, t3655: f64, t6598: f64, t6602: f64, t12705: f64, t12712: f64, t12866: f64, t17351: f64, t17353: f64, t17638: f64, t21020: f64, t21040: f64, t225: f64, t3625: f64, t3626: f64, t3630: f64, t44585: f64, t44704: f64, t480: f64, t484: f64, t56879: f64, t57548: f64, t57550: f64, t57606: f64, t58850: f64, t58853: f64, t60927: f64, t6638: f64, t71112: f64, t71117: f64) -> (f64, f64) {
    let t71179 = (t71134 + t71148 + t71162 + t71176) * t459;
    let t71187 = t6598 * t3655;
    let t71192 = t6602 * t3655;
    let t71196 = -0.84689418504829110067e-4_f64 * t44704 + 0.28582678745379824648e-3_f64 * t17351 * t17353 * t12705 * t6638 + 0.57165357490759649296e-3_f64 * t17351 * t17353 * t12712 * t21020 - 0.28582678745379824648e-3_f64 * t56879 * t17353 * t44585 * t6638 + 0.28582678745379824648e-3_f64 * t12866 * t71112 * t3630 + 0.19055119163586549765e-3_f64 * t71117 - 0.14291339372689912324e-3_f64 * t3625 * t3626 * t21040 * t17638 + 0.21437009059034868486e-3_f64 * t71179 * t225 * t480 * t484 - 7.0_f64 / 162.0_f64 * t57548 * t57550 * t60927 + 0.5081365110289746604e-3_f64 * t71187 + t57548 * t57606 * t60927 / 9.0_f64 - 0.47637797908966374413e-4_f64 * t71192 + 0.19055119163586549765e-3_f64 * t58850 - 0.28582678745379824648e-3_f64 * t58853;
    (t71179, t71196)
}
