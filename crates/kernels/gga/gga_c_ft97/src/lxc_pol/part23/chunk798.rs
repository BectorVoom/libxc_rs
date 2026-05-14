//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 798/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk798<F: Float>(t24237: F, t6005: F, t263: F, t6061: F, t1424: F, t771: F, t5996: F, t5999: F, t1410: F, t2426: F) -> (F, F, F, F, F) {
    let t24238 = t24237 * t6005;
    let t24240 = t6061 * t263;
    let t24245 = t1424 * t771;
    let t24253 = t5996 * t5999;
    let t24260 = t2426 * t1410;
    (t24238, t24240, t24245, t24253, t24260)
}
