//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 993/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk993<F: Float>(t153381: F, t2665: F, t446: F, t505: F, t143187: F, t143204: F, t143245: F, t143264: F, t152948: F, t152952: F, t152954: F, t152958: F, t152962: F, t152965: F, t152970: F, t152975: F, t152979: F, t153375: F, t153379: F) -> (F, F) {
    let t153384 = t446 * t2665 * t153381 * t505;
    let t153386 = 8.0 * t152948 - 4.0 * t152952 + 2.0 / 27.0 * t152954 - 2.0 / 9.0 * t152958 + t143187 / 18.0 - 8.0 / 9.0 * t152962 - 4.0 / 9.0 * t152965 - 2.0 / 9.0 * t143204 - t143245 / 9.0 - 4.0 / 9.0 * t152970 + 2.0 / 3.0 * t152975 + 2.0 / 3.0 * t152979 + t143264 - t153375 / 6.0 - 4.0 * t153379 + t153384 / 9.0;
    (t153384, t153386)
}
