//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 947/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk947<F: Float>(t10432: F, t2465: F, t2787: F, t2464: F, t2487: F, t2778: F, t587: F, t1407: F, t3396: F, t10430: F, t912: F, t2293: F, t2854: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10433 = F::cast_from(0.19171462976960374838e0_f64) * t10432;
    let t10434 = t2465 * t2787;
    let t10435 = t2464 * t10434;
    let t10436 = t2487 * t10435;
    let t10437 = F::cast_from(0.42603251059911944084e-1_f64) * t10436;
    let t10438 = t2465 * t2778;
    let t10439 = t2464 * t10438;
    let t10440 = t587 * t10439;
    let t10441 = F::cast_from(0.42603251059911944084e-1_f64) * t10440;
    let t10442 = t1407 * t3396;
    let t10443 = F::cast_from(0.19171462976960374838e0_f64) * t10442;
    let t10444 = t912 * t10430;
    let t10445 = t587 * t10444;
    let t10446 = F::cast_from(0.19171462976960374838e0_f64) * t10445;
    let t10447 = t2854 * t2293;
    (t10433, t10434, t10435, t10437, t10438, t10439, t10441, t10443, t10444, t10446, t10447)
}
