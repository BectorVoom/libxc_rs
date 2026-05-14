//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1106/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1106<F: Float>(t2426: F, t6776: F, t108817: F, t14116: F, t2446: F, t2427: F, t3758: F, t24275: F, t13411: F, t3722: F, t3724: F, t6032: F, t209: F, t8: F, t420: F, t1689: F, t30815: F, t6789: F, t6793: F) -> (F, F, F, F, F, F) {
    let t109216 = t2426 * t6776;
    let t109221 = t108817 * t2446 * t14116;
    let t109230 = t3758 * t2427;
    let t109231 = t109230 * t24275;
    let t109245 = t13411 * t3722 * t3724 * t6032;
    let t109246 = t8 * t209;
    let t109247 = t109246 * t420;
    let t109254 = t30815 * t1689 * t6789 * t6793;
    (t109216, t109221, t109231, t109245, t109247, t109254)
}
