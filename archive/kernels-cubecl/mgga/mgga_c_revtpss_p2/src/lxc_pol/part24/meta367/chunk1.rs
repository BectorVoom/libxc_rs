//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1249/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1249<F: Float>(t24285: F, t24322: F, t1150: F, t1131: F, t12230: F, t24220: F, t12227: F, t1744: F, t6486: F, t3479: F, t16706: F, t16876: F, t20276: F, t20278: F, t20280: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F, t24265: F, t24267: F, t24272: F, t24275: F) -> (F, F, F, F, F, F, F, F) {
    let t24323 = t24285 + t24322;
    let t24324 = t24323 * t1150;
    let t24326 = F::cast_from(1.0_f64) * t1131 * t24324;
    let t24327 = t24220 * t12230;
    let t24329 = F::cast_from(0.51726012919273400301e3_f64) * t12227 * t24327;
    let t24330 = t6486 * t1744;
    let t24331 = t24330 * t3479;
    let t24348 = -F::cast_from(0.52945875e1_f64) * t24265 + F::cast_from(0.94674375e0_f64) * t24267 + F::cast_from(0.68863333333333333332e0_f64) * t16706 + F::cast_from(0.34731666666666666667e0_f64) * t16876 + F::cast_from(0.46308888888888888889e-1_f64) * t24272 + F::cast_from(0.62517e0_f64) * t24275 + F::cast_from(0.69463333333333333335e-1_f64) * t20276 - F::cast_from(0.41678000000000000001e0_f64) * t20278 - F::cast_from(0.20839e0_f64) * t20280 + F::cast_from(0.34431666666666666666e0_f64) * t20283 - F::cast_from(0.103295e1_f64) * t20285 - F::cast_from(0.51647499999999999999e0_f64) * t20287 + F::cast_from(0.57386111111111111112e0_f64) * t24230 - F::cast_from(0.20659e1_f64) * t24234;
    (t24323, t24324, t24326, t24327, t24329, t24330, t24331, t24348)
}
