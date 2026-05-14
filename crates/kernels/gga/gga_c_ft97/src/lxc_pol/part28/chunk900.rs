//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 900/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk900<F: Float>(t34406: F, t376: F, t5665: F, t32063: F, t34380: F, t7238: F, t144853: F, t1564: F, t446: F, t144822: F, t7793: F, t34482: F, t432: F, t1317: F, t1800: F, t28: F) -> (F, F, F, F, F, F) {
    let t144946 = t5665 * t376 * t34406;
    let t144950 = t7238 * t32063 * t34380;
    let t144953 = t446 * t1564 * t144853;
    let t144956 = t446 * t7793 * t144822;
    let t144958 = t34482 * t432;
    let t144961 = t1317 * t28 * t1800 * t144958;
    (t144946, t144950, t144953, t144956, t144958, t144961)
}
