//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 931/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk931<F: Float>(t10426: F, t1820: F, t3429: F, t562: F, t1821: F, t610: F, t1827: F, t587: F, t1764: F, t3346: F, t418: F, t1663: F) -> (F, F, F, F, F, F) {
    let t10428 = F::new(8.0) / F::new(45.0) * t1820 * t10426;
    let t10429 = t3429 * t562;
    let t10430 = t1821 * t10429;
    let t10432 = F::new(8.0) / F::new(45.0) * t1820 * t10430;
    let t10433 = t3429 * t610;
    let t10434 = t1827 * t10433;
    let t10436 = F::new(4.0) / F::new(45.0) * t587 * t10434;
    let t10437 = t1764 * t3346;
    let t10438 = t10437 * t418;
    let t10439 = t1821 * t10438;
    let t10441 = F::new(8.0) / F::new(45.0) * t587 * t10439;
    let t10442 = t1663 * t3346;
    (t10428, t10432, t10436, t10438, t10441, t10442)
}
