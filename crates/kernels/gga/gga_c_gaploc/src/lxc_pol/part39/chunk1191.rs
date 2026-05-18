//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1191/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1191<F: Float>(t41984: F, t41987: F, t41989: F, t41991: F, t41992: F, t41996: F, t42001: F, t42005: F, t42008: F, t42015: F, t42018: F, t42022: F) -> F {
    let t48010 = t41984 - t41987 - t41989 + t41991 + t41992 - t41996 - F::new(0.29792074959875355558e-1) * t42001 + t42005 + t42008 - F::new(0.69017266717057349418e1) * t42015 - t42018 - t42022;
    t48010
}
