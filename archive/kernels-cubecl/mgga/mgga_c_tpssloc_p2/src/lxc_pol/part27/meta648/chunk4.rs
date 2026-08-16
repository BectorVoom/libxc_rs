//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2242/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2242<F: Float>(t25471: F, t82431: F, t7607: F, t82632: F, t25490: F, t82514: F, t23518: F, t7577: F, t1014: F, t1023: F, t1049: F, t12648: F, t12652: F, t23327: F, t23601: F, t23602: F, t23605: F, t23633: F, t23705: F, t23714: F, t25429: F, t25470: F, t25485: F, t25491: F, t25492: F, t25510: F, t25554: F, t25721: F, t3041: F, t3121: F, t4669: F, t4677: F, t6743: F, t82513: F, t82809: F, t89194: F, t89205: F) -> F {
    let t89445 = F::cast_from(0.18277045187202515961e-2_f64) * t82431 * t25471;
    let t89449 = t82632 * t7607;
    let t89468 = t82514 * t25490;
    let t89473 = t7577 * t23518;
    let t89477 = -F::cast_from(0.36554090374405031922e-2_f64) * t82809 + t4669 * t23705 + F::cast_from(0.73108180748810063846e-2_f64) * t25429 * t25510 * t25721 * t12652 + F::cast_from(0.36554090374405031923e-2_f64) * t25429 * t25510 * t25721 * t12648 - t89445 + F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t25470 * t23714 + F::cast_from(0.18277045187202515961e-2_f64) * t89449 + F::cast_from(0.54831135561607547884e-2_f64) * t23633 * t6743 * t4677 * t25554 - F::cast_from(0.16449340668482264365e-1_f64) * t23601 * t23602 * t1014 * t1049 * t25492 - F::cast_from(0.16449340668482264365e-1_f64) * t23601 * t25491 * t89194 * t1023 - F::cast_from(0.82246703342411321825e-2_f64) * t23601 * t25491 * t25485 * t3121 + F::cast_from(0.82246703342411321825e-2_f64) * t82513 * t89468 * t89205 * t3041 + F::cast_from(0.82246703342411321825e-2_f64) * t23601 * t89473 * t23605;
    t89477
}
