//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 737/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk737<F: Float>(t39: F, t5772: F, t5773: F, t497: F, t542: F, t496: F, t120: F, t1508: F, t19: F, t5763: F, t119: F, t331: F, t481: F, t1557: F, t1513: F, t505: F, t96: F) -> (F, F, F, F, F, F, F) {
    let t5776 = 0.19486833333333333333e1 * t5772 * t5773 * t39;
    let t5783 = t542 * t497;
    let t5784 = t496 * t5783;
    let t5795 = t1508 * t120 * t19;
    let t5796 = t5795 * t5763;
    let t5809 = t119 * t331 * t481;
    let t5810 = t1557 * t5809;
    let t5816 = t1513 * t5809;
    let t5825 = 1.0 / t505 / t96;
    (t5776, t5783, t5784, t5796, t5810, t5816, t5825)
}
