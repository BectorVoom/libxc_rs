//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 841/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk841<F: Float>(t22862: F, t469: F, t1317: F, t28: F, t376: F, t5684: F, t1307: F, t1570: F, t1559: F, t1564: F, t446: F, t1318: F, t1637: F, t22873: F, t432: F, t89: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t23024 = t469 * t22862;
    let t23026 = t1317 * t28 * t23024;
    let t23029 = t1317 * t376 * t5684;
    let t23031 = t1307 * t1570;
    let t23032 = t23031 * t1559;
    let t23033 = t1564 * t23032;
    let t23034 = t446 * t23033;
    let t23037 = t1317 * t1637 * t1318;
    let t23038 = 2.0 / 9.0 * t23037;
    let t23039 = t22873 * t432;
    let t23040 = t28 * t23039;
    let t23041 = t89 * t23040;
    (t23024, t23026, t23029, t23031, t23033, t23034, t23037, t23038, t23039, t23041)
}
