//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1189/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1189<F: Float>(t13801: F, t1641: F, t41960: F, t41962: F, t41968: F, t41970: F, t41972: F, t41973: F, t47976: F, t47978: F, t47980: F, t47984: F, t47987: F) -> F {
    let t47989 = t1641 * t13801;
    let t47992 = F::new(0.14896037479937677779e-1) * t41960 + F::new(0.14896037479937677779e-1) * t41962 + F::new(0.14896037479937677779e-1) * t47976 + F::new(0.14896037479937677779e-1) * t47978 - F::new(0.14896037479937677779e-1) * t47980 - F::new(0.14896037479937677779e-1) * t47984 - F::new(0.71500979903700853338e0) * t47987 - F::new(0.46011511144704899612e1) * t47989 + t41968 + F::new(0.46011511144704899612e1) * t41970 - t41972 - t41973;
    t47992
}
