//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 520/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk520<F: Float>(t1356: F, t1387: F, t1413: F, t1418: F, t2045: F, t2052: F, t2059: F, t2063: F, t2451: F, t2453: F, t2455: F, t2465: F, t2485: F, t2487: F, t2488: F, t2810: F, t2813: F, t2816: F, t765: F) -> F {
    let t2819 = F::cast_from(0.285764e-1_f64) * t2045 + t2052 - t2059 - F::cast_from(0.675260332e-1_f64) * t2063 + t1356 + t2451 - t2453 - t2455 + t2465 - t2485 + F::cast_from(0.675260332e-1_f64) * t765 * t2810 + F::cast_from(0.675260332e-1_f64) * t765 * t2813 + F::cast_from(0.675260332e-1_f64) * t765 * t2816 + t2487 + t1387 + t2488 + t1413 - t1418;
    t2819
}
