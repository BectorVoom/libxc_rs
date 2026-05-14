//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1268/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1268<F: Float>(t113120: F, t113124: F, t113128: F, t113131: F, t113135: F, t113139: F, t113144: F, t113147: F, t113150: F, t113154: F, t113159: F, t113164: F, t113168: F, t113176: F, t113195: F, t113201: F) -> (F, F, F, F, F) {
    let t114311 = t113120 / 27.0 - 4.0 * t113124 - 4.0 / 9.0 * t113128 - 4.0 / 9.0 * t113131 - 2.0 * t113135 + t113139 / 9.0 + 2.0 / 9.0 * t113144 + 2.0 / 27.0 * t113147 + 10.0 / 81.0 * t113150 - 8.0 / 27.0 * t113154 + t113159 / 12.0 + t113164 / 6.0;
    let t114312 = 2.0 / 9.0 * t113168;
    let t114314 = 2.0 / 9.0 * t113176;
    let t114318 = t113195 / 54.0;
    let t114320 = t113201 / 54.0;
    (t114311, t114312, t114314, t114318, t114320)
}
