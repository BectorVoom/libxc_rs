//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1164/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1164<F: Float>(t116561: F, t1317: F, t1800: F, t28: F, t101598: F, t102166: F, t102173: F, t102175: F, t116546: F, t116549: F, t116552: F, t116555: F, t116560: F, t93425: F, t101734: F, t3204: F, t93378: F, t93379: F) -> (F, F, F) {
    let t116564 = t1317 * t28 * t1800 * t116561;
    let t116566 = -t102166 - t93425 + 2.0 * t116546 + 2.0 * t116549 - 4.0 / 3.0 * t116552 - 8.0 / 3.0 * t116555 + t116560 + t116564 - t102173 - t102175 + 2.0 / 9.0 * t101598;
    let t116569 = t93378 * t93379 * t101734 * t3204;
    (t116564, t116566, t116569)
}
