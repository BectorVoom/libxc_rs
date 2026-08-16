//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 742/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk742<F: Float>(t1546: F, t4426: F, t89: F, t4432: F, t1586: F, t4495: F, t432: F, t28: F, t3013: F, t3103: F, t1577: F, t7743: F) -> (F, F, F, F, F) {
    let t15609 = t89 * t1546 * t4426;
    let t15612 = t89 * t1546 * t4432;
    let t15614 = t1586 * t4495;
    let t15615 = t15614 * t432;
    let t15617 = t89 * t28 * t15615;
    let t15619 = t3013 * t3103;
    let t15621 = t89 * t28 * t15619;
    let t15625 = -F::cast_from(2.0_f64) * t1577 - F::cast_from(6.0_f64) * t7743;
    (t15609, t15612, t15617, t15621, t15625)
}
