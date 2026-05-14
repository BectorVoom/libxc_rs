//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 833/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk833<F: Float>(t1445: F, t1486: F, t1481: F, t3783: F, t3507: F, t4229: F, t14187: F, t492: F, t13328: F, t484: F, t380: F, t470: F, t140: F, t446: F, t480: F, t1460: F, t306: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14304 = t1486 * t1445;
    let t14320 = t1481 * t3783;
    let t14321 = t14320 * sigma0;
    let t14344 = t3507 * t4229;
    let t14356 = t14187 * t492;
    let t14364 = t484 * t13328;
    let t14374 = 1.0 / t470 / t380;
    let t14409 = 0.11791604938271604938e-1 * t140 * t446 * t480;
    let t14434 = t1460 * t306;
    (t14304, t14320, t14321, t14344, t14356, t14364, t14374, t14409, t14434)
}
