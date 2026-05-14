//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 848/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk848<F: Float>(t1491: F, t7570: F, t8281: F, t34284: F, t34287: F, t25462: F, t34003: F, t33998: F, t25485: F, t7581: F, t1466: F, t34021: F, t681: F, t25488: F, t34253: F, t34277: F) -> (F, F, F, F, F, F, F, F, F) {
    let t142576 = 2.0 / 27.0 * t7570 * t8281 * t1491;
    let t142577 = t34284 * t34287;
    let t142595 = t25462 * t34003;
    let t142597 = t25462 * t33998;
    let t142602 = 2.0 / 27.0 * t7581 * t25485;
    let t142611 = t1466 * t681 * t34021;
    let t142613 = t7581 * t25488;
    let t142618 = t1466 * t681 * t34253;
    let t142636 = t1466 * t681 * t34277;
    (t142576, t142577, t142595, t142597, t142602, t142611, t142613, t142618, t142636)
}
