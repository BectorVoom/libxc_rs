//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1075/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1075<F: Float>(t120108: F, t120120: F, t120139: F, t122004: F, t122008: F, t122009: F, t122010: F, t122015: F, t126365: F, t126376: F, t32445: F, t34075: F, t126388: F, t110687: F, t120149: F, t122024: F, t122026: F, t126378: F, t126384: F, t126386: F, t127739: F, t26550: F, t27299: F, t27317: F, t27358: F, t32434: F, t32463: F, t41077: F, t7078: F, t8477: F, t8648: F) -> (F, F) {
    let t127847 = 0.25702851531048074406e-1 * t122004 - 0.17135921299530705785e1 * t34075 * t32445 + t120108 - t122008 + t122009 - t122010 - t120120 - t122015 - 0.56468933516960933999e-3 * t126365 + t120139 + 0.37645955677973955999e-4 * t126376;
    let t127858 = 0.263521689745817692e-2 * t126388;
    let t127868 = -0.66934509195437693771e-4 * t126378 - 0.11423947533020470523e1 * t32463 * t110687 * t7078 - 0.17347256376410398924e1 * t127739 * t27358 + 0.26447628533477078895e-3 * t120149 + 0.14874931683620404328e-2 * t126384 + 0.7437465841810202164e-3 * t126386 + t127858 - 0.76169170176413987214e-1 * t122024 + 0.25389723392137995738e-1 * t122026 + 0.6854368519812282314e1 * t8477 * t8648 * t41077 * t26550 * t27299 + 0.17347256376410398924e1 * t32434 * t27317;
    (t127847, t127868)
}
