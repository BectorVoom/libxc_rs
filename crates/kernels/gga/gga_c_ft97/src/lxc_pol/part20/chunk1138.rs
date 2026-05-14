//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1138/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1138<F: Float>(t108171: F, t108210: F, t108168: F, t108176: F, t108182: F, t108191: F, t108195: F, t108200: F, t108204: F, t108208: F, t108215: F, t108220: F, t108224: F, t108229: F, t108233: F, t108238: F, t96926: F, t96940: F, t96951: F, t96953: F, t96955: F, t96958: F, t96968: F) -> (F, F) {
    let t110095 = 2.0 / 9.0 * t108171;
    let t110103 = 2.0 / 27.0 * t108210;
    let t110105 = 2.0 / 3.0 * t108168 - t110095 + t108176 / 6.0 - t108182 / 27.0 + t108191 / 12.0 + t108195 / 9.0 + t108200 / 24.0 - t108204 / 18.0 - 2.0 / 9.0 * t108208 + t110103 + t108215 / 2.0;
    let t110118 = -2.0 / 9.0 * t108220 + t96926 / 54.0 + t108224 / 9.0 - t108229 / 6.0 - t108233 / 8.0 - t108238 / 4.0 - t96940 / 27.0 + t96951 / 18.0 + 8.0 / 81.0 * t96953 - 2.0 / 81.0 * t96955 - 4.0 / 27.0 * t96958 - 2.0 / 27.0 * t96968;
    (t110105, t110118)
}
