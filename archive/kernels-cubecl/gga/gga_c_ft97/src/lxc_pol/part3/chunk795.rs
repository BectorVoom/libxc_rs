//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 795/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk795<F: Float>(t15606: F, t15609: F, t15612: F, t15891: F, t15894: F, t15604: F, t15617: F, t15621: F, t15628: F, t15888: F, t15897: F, t15899: F) -> (F, F) {
    let t16336 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t15606;
    let t16337 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t15609;
    let t16338 = t15612 / F::cast_from(9.0_f64);
    let t16342 = t15891 / F::cast_from(3.0_f64);
    let t16343 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t15894;
    let t16345 = -F::cast_from(6.0_f64) * t15604 + t16336 - t16337 + t16338 + F::cast_from(2.0_f64) * t15617 + F::cast_from(4.0_f64) * t15621 - t15628 / F::cast_from(3.0_f64) - t15888 + t16342 - t16343 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t15897;
    let t16346 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t15899;
    (t16345, t16346)
}
