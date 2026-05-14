//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 645/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk645<F: Float>(t14348: F, t502: F, t1052: F, t3749: F, t3009: F, t3720: F, t1445: F, t12256: F, t13045: F, t13591: F, t13595: F, t13597: F, t13600: F, t13604: F, t13606: F, t13608: F, t13611: F, t13849: F, t13852: F, t2087: F) -> (F, F, F, F, F) {
    let t14349 = t502 * t14348;
    let t14350 = t1052 * t3749;
    let t14357 = t3009 * t3720;
    let t14358 = t1445 * t14357;
    let t14361 = t13591 - t13595 + t13597 + t13600 - t13604 + 0.76685851907841499354e0 * t13849 - 0.76685851907841499354e0 * t13852 - 0.21450293971110256002e1 * t12256 * t13045 - 0.13803453343411469884e2 * t2087 * t14358 - t13606 - t13608 + t13611;
    (t14349, t14350, t14357, t14358, t14361)
}
