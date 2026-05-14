//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1217/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1217<F: Float>(t100292: F, t100294: F, t100307: F, t100309: F, t100311: F, t100313: F, t100298: F, t100302: F, t100305: F, t100317: F, t100321: F, t100325: F, t100333: F, t100338: F, t100343: F, t100347: F, t100351: F, t100358: F, t100362: F, t100367: F, t92252: F, t92255: F, t92259: F) -> (F, F) {
    let t102114 = t100292 / 4.0;
    let t102115 = 2.0 / 9.0 * t100294;
    let t102119 = t100307 / 9.0;
    let t102120 = 4.0 / 9.0 * t100309;
    let t102121 = 4.0 / 9.0 * t100311;
    let t102122 = 4.0 / 27.0 * t100313;
    let t102126 = t102114 + t102115 - t100298 / 6.0 + 12.0 * t100302 - 11.0 / 9.0 * t100305 - t102119 + t102120 + t102121 - t102122 - 4.0 / 3.0 * t100317 - 4.0 / 9.0 * t100321 - 4.0 / 3.0 * t100325;
    let t102134 = t100333 / 2.0 + t100338 + t100343 - 2.0 / 3.0 * t100347 + t100351 + t92252 + 4.0 / 3.0 * t100358 - 4.0 / 9.0 * t100362 + t92255 - t92259 - t100367 / 3.0;
    (t102126, t102134)
}
