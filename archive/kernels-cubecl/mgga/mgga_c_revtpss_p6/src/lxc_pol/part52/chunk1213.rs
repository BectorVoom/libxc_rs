//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1213/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1213<F: Float>(t126388: F, t110687: F, t120149: F, t122024: F, t122026: F, t126378: F, t126384: F, t126386: F, t127739: F, t26550: F, t27299: F, t27317: F, t27358: F, t32434: F, t32463: F, t41077: F, t7078: F, t8477: F, t8648: F) -> F {
    let t127858 = F::cast_from(0.263521689745817692e-2_f64) * t126388;
    let t127868 = -F::cast_from(0.66934509195437693771e-4_f64) * t126378 - F::cast_from(0.11423947533020470523e1_f64) * t32463 * t110687 * t7078 - F::cast_from(0.17347256376410398924e1_f64) * t127739 * t27358 + F::cast_from(0.26447628533477078895e-3_f64) * t120149 + F::cast_from(0.14874931683620404328e-2_f64) * t126384 + F::cast_from(0.7437465841810202164e-3_f64) * t126386 + t127858 - F::cast_from(0.76169170176413987214e-1_f64) * t122024 + F::cast_from(0.25389723392137995738e-1_f64) * t122026 + F::cast_from(0.6854368519812282314e1_f64) * t8477 * t8648 * t41077 * t26550 * t27299 + F::cast_from(0.17347256376410398924e1_f64) * t32434 * t27317;
    t127868
}
