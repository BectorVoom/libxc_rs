//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 853/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk853<F: Float>(t43407: F, t2617: F, t3621: F, t7803: F, t43412: F, t43416: F, t15499: F, t3601: F, t2679: F, t28640: F, t10827: F, t3005: F, t9800: F) -> (F, F, F, F, F, F) {
    let t45195 = F::new(0.92023022289409799224e1) * t43407;
    let t45197 = t7803 * t3621 * t2617;
    let t45199 = F::new(0.15337170381568299871e1) * t43412;
    let t45200 = F::new(0.15337170381568299871e1) * t43416;
    let t45209 = t15499 * t3601;
    let t45211 = t28640 * t45209 * t2679;
    let t45212 = F::new(0.23005755572352449806e1) * t45211;
    let t45214 = t9800 * t3005 * t10827;
    (t45195, t45197, t45199, t45200, t45212, t45214)
}
