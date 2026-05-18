//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1226/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1226<F: Float>(t39310: F, t4190: F, t8186: F, t27553: F, t5897: F, t4188: F, t8182: F, t4189: F, t6048: F, t7962: F, t28450: F, t4142: F) -> (F, F, F, F, F) {
    let t97989 = F::new(24.0) * t39310 * t8186 * t4190;
    let t97990 = t5897 * t27553;
    let t97991 = t8182 * t4188;
    let t97993 = F::new(2.0) * t97991 * t4190;
    let t97996 = F::new(4.0) * t4189 * t7962 * t6048;
    let t97997 = t4142 * t28450;
    (t97989, t97990, t97993, t97996, t97997)
}
