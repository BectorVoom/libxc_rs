//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1119/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1119<F: Float>(t109442: F, t1934: F, t2354: F, t27468: F, t446: F, t2347: F, t6837: F, t2349: F, t9744: F, t96960: F, t992: F, t10157: F, t27841: F, t713: F, t2459: F, t6852: F) -> (F, F, F, F, F, F) {
    let t109443 = t109442 / 3.0;
    let t109446 = t446 * t2354 * t27468 * t1934;
    let t109448 = t6837 * t2347;
    let t109451 = t446 * t9744 * t109448 * t2349;
    let t109455 = t446 * t2354 * t96960 * t992;
    let t109459 = t446 * t10157 * t27841 * t713;
    let t109463 = t446 * t10157 * t6852 * t2459;
    (t109443, t109446, t109451, t109455, t109459, t109463)
}
