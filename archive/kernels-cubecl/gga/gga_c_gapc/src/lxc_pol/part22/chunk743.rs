//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 743/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk743<F: Float>(t8566: F, t8567: F, t2952: F, t4015: F, t4018: F, t8362: F, t1595: F, t2890: F, t473: F, t1037: F, t1416: F, t4687: F) -> (F, F, F, F, F) {
    let t8568 = t8566 * t8567;
    let t8570 = t2952 * t4015;
    let t8571 = t8362 * t4018;
    let t8572 = t8570 * t8571;
    let t8574 = t2890 * t1595;
    let t8575 = t473 * t8574;
    let t8577 = t1416 * t1037;
    let t8578 = t8577 * t4687;
    (t8568, t8570, t8572, t8575, t8578)
}
