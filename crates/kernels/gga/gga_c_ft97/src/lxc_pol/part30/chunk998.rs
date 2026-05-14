//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 998/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk998<F: Float>(t1486: F, t35849: F, t681: F, t152776: F, t33829: F, t7512: F, t7638: F, t152780: F, t7641: F, t152826: F, t193: F, t2781: F, t6308: F, t143329: F, t143333: F, t143336: F, t143339: F, t143355: F, t143366: F, t143371: F, t153435: F, t153439: F, t153443: F, t153449: F, t153453: F) -> (F, F, F, F, F) {
    let t153456 = t1486 * t681 * t35849;
    let t153460 = t7638 * t7512 * t33829 * t152776;
    let t153464 = t7638 * t7512 * t7641 * t152780;
    let t153468 = t6308 * t193 * t2781 * t152826;
    let t153470 = -2.0 * t153435 - t153439 / 6.0 - t153443 / 9.0 + 2.0 / 27.0 * t143329 + t143333 - t143336 + t143339 / 9.0 - t143355 / 36.0 + t153449 / 3.0 - t143366 - t143371 / 27.0 + 2.0 / 27.0 * t153453 - 2.0 / 9.0 * t153456 + 4.0 / 3.0 * t153460 - 2.0 / 3.0 * t153464 - t153468 / 6.0;
    (t153456, t153460, t153464, t153468, t153470)
}
