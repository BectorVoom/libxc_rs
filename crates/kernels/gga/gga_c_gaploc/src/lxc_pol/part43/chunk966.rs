//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 966/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk966<F: Float>(t47243: F, t7427: F, t7573: F, t12223: F, t1445: F, t2530: F, t813: F, t13870: F, t2089: F, t2087: F, t723: F, t13865: F, t4614: F) -> (F, F, F, F) {
    let t47245 = t7427 * t7573 * t47243;
    let t47255 = t813 * t1445 * t12223 * t2530;
    let t47257 = t2089 * t13870;
    let t47261 = F::cast_from(0.69017266717057349418e1_f64) * t2087 * t1445 * t47257 * t723;
    let t47263 = t2087 * t4614 * t13865;
    (t47245, t47255, t47261, t47263)
}
