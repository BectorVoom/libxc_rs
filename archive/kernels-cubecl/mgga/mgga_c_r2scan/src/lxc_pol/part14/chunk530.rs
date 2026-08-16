//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 530/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk530<F: Float>(t2483: F, t88: F, t41: F, t410: F, t899: F, t1388: F, t1356: F, t1387: F, t1413: F, t1418: F, t1421: F, t1511: F, t2451: F, t2453: F, t2455: F, t2465: F) -> (F, F, F, F, F) {
    let t2484 = t2483 * t88;
    let t2485 = t41 * t2484;
    let t2486 = t410 * t899;
    let t2487 = F::cast_from(4.0_f64) * t2486;
    let t2488 = F::cast_from(0.5848223622634646207e0_f64) * t1388;
    let t2489 = -t1356 - t2451 + t2453 + t2455 - t2465 + t2485 - t2487 - t1387 - t2488 - t1413 + t1418 - t1421 + t1511;
    (t2484, t2485, t2487, t2488, t2489)
}
