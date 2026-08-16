//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1216/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1216(t126388: f64, t110687: f64, t120149: f64, t122024: f64, t122026: f64, t126378: f64, t126384: f64, t126386: f64, t127739: f64, t26550: f64, t27299: f64, t27317: f64, t27358: f64, t32434: f64, t32463: f64, t41077: f64, t7078: f64, t8477: f64, t8648: f64) -> f64 {
    let t127858 = 0.263521689745817692e-2_f64 * t126388;
    let t127868 = -0.66934509195437693771e-4_f64 * t126378 - 0.11423947533020470523e1_f64 * t32463 * t110687 * t7078 - 0.17347256376410398924e1_f64 * t127739 * t27358 + 0.26447628533477078895e-3_f64 * t120149 + 0.14874931683620404328e-2_f64 * t126384 + 0.7437465841810202164e-3_f64 * t126386 + t127858 - 0.76169170176413987214e-1_f64 * t122024 + 0.25389723392137995738e-1_f64 * t122026 + 0.6854368519812282314e1_f64 * t8477 * t8648 * t41077 * t26550 * t27299 + 0.17347256376410398924e1_f64 * t32434 * t27317;
    t127868
}
