//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1066/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1066<F: Float>(t100292: F, t100294: F, t100307: F, t100309: F, t100311: F, t100313: F, t100409: F, t100430: F, t100477: F, t100479: F, t100481: F, t101587: F, t101595: F, t101615: F, t101661: F, t101687: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t102114 = t100292 / 4.0;
    let t102115 = 2.0 / 9.0 * t100294;
    let t102119 = t100307 / 9.0;
    let t102120 = 4.0 / 9.0 * t100309;
    let t102121 = 4.0 / 9.0 * t100311;
    let t102122 = 4.0 / 27.0 * t100313;
    let t102144 = t100409 / 6.0;
    let t102151 = 2.0 / 3.0 * t100430;
    let t102164 = 2.0 / 9.0 * t100477;
    let t102165 = 2.0 / 9.0 * t100479;
    let t102166 = 2.0 / 27.0 * t100481;
    let t102173 = t101587 / 6.0;
    let t102175 = 2.0 / 3.0 * t101595;
    let t102181 = t101615 / 18.0;
    let t102193 = t101661 / 3.0;
    let t102202 = t101687 / 3.0;
    (t102114, t102115, t102119, t102120, t102121, t102122, t102144, t102151, t102164, t102165, t102166, t102173, t102175, t102181, t102193, t102202)
}
