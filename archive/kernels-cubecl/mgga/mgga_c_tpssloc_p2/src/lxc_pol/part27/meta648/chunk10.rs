//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2248/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2248<F: Float>(t23384: F, t25802: F, t23587: F, t7560: F, t25410: F, t1052: F, t14548: F, t23341: F, t23346: F, t23394: F, t25436: F, t25797: F, t3016: F, t3174: F, t3206: F, t4557: F, t6687: F, t6704: F, t7561: F, t7624: F, t83435: F, t83441: F, t83444: F, t89349: F, t986: F) -> F {
    let t89630 = F::cast_from(0.18277045187202515961e-2_f64) * t23384 * t25802;
    let t89648 = t7560 * t23587;
    let t89653 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25410;
    let t89658 = -F::cast_from(0.27415567780803773942e-2_f64) * t83435 - F::cast_from(0.48738787165873375897e-2_f64) * t83441 - F::cast_from(0.14621636149762012769e-1_f64) * t23346 * t25802 + t89630 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t6704 * t23394 * t14548 + F::cast_from(2.0_f64) * t1052 * t3174 * t7624 * t3206 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t3016 * t7561 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t89349 * t25797 - F::cast_from(0.36554090374405031922e-2_f64) * t83444 - F::cast_from(6.0_f64) * t4557 * t23341 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t986 * t89648 - t89653 + F::cast_from(0.43864908449286038306e-1_f64) * t23346 * t25410 - F::cast_from(0.14621636149762012769e-1_f64) * t23346 * t25436;
    t89658
}
