//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 912/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk912<F: Float>(t41234: F, t43777: F, t43778: F, t43781: F, t43783: F, t43787: F, t43790: F, t43793: F, t43800: F, t43803: F, t43806: F, t43809: F, t43820: F, t43822: F, t43830: F, t43833: F, t47379: F, t47381: F, t47389: F) -> (F,) {
    let t51142 = -0.59584149919750711115e-1 * t41234 + t43777 - t43778 + 0.29792074959875355558e-1 * t47379 + 0.29792074959875355558e-1 * t47381 + t43781 - t43783 - 0.51123901271894332901e0 * t47389 - t43787 + t43790 + t43793 + t43800 - t43803 + t43806 - t43809 + t43820 + t43822 - t43830 + t43833;
    (t51142,)
}
