//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 795/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk795<F: Float>(t15606: F, t15609: F, t15612: F, t15891: F, t15894: F, t15604: F, t15617: F, t15621: F, t15628: F, t15888: F, t15897: F, t15899: F) -> (F, F) {
    let t16336 = F::new(2.0) / F::new(27.0) * t15606;
    let t16337 = F::new(2.0) / F::new(9.0) * t15609;
    let t16338 = t15612 / F::new(9.0);
    let t16342 = t15891 / F::new(3.0);
    let t16343 = F::new(2.0) / F::new(3.0) * t15894;
    let t16345 = -F::new(6.0) * t15604 + t16336 - t16337 + t16338 + F::new(2.0) * t15617 + F::new(4.0) * t15621 - t15628 / F::new(3.0) - t15888 + t16342 - t16343 - F::new(8.0) / F::new(9.0) * t15897;
    let t16346 = F::new(2.0) / F::new(9.0) * t15899;
    (t16345, t16346)
}
