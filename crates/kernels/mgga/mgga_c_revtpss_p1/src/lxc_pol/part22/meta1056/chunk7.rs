//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3744/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3744<F: Float>(t459: F, t71134: F, t71148: F, t71162: F, t71176: F, t3655: F, t6598: F, t6602: F, t12705: F, t12712: F, t12866: F, t17351: F, t17353: F, t17638: F, t21020: F, t21040: F, t225: F, t3625: F, t3626: F, t3630: F, t44585: F, t44704: F, t480: F, t484: F, t56879: F, t57548: F, t57550: F, t57606: F, t58850: F, t58853: F, t60927: F, t6638: F, t71112: F, t71117: F) -> (F, F) {
    let t71179 = (t71134 + t71148 + t71162 + t71176) * t459;
    let t71187 = t6598 * t3655;
    let t71192 = t6602 * t3655;
    let t71196 = -F::cast_from(0.84689418504829110067e-4_f64) * t44704 + F::cast_from(0.28582678745379824648e-3_f64) * t17351 * t17353 * t12705 * t6638 + F::cast_from(0.57165357490759649296e-3_f64) * t17351 * t17353 * t12712 * t21020 - F::cast_from(0.28582678745379824648e-3_f64) * t56879 * t17353 * t44585 * t6638 + F::cast_from(0.28582678745379824648e-3_f64) * t12866 * t71112 * t3630 + F::cast_from(0.19055119163586549765e-3_f64) * t71117 - F::cast_from(0.14291339372689912324e-3_f64) * t3625 * t3626 * t21040 * t17638 + F::cast_from(0.21437009059034868486e-3_f64) * t71179 * t225 * t480 * t484 - F::new(7.0) / F::new(162.0) * t57548 * t57550 * t60927 + F::cast_from(0.5081365110289746604e-3_f64) * t71187 + t57548 * t57606 * t60927 / F::new(9.0) - F::cast_from(0.47637797908966374413e-4_f64) * t71192 + F::cast_from(0.19055119163586549765e-3_f64) * t58850 - F::cast_from(0.28582678745379824648e-3_f64) * t58853;
    (t71179, t71196)
}
