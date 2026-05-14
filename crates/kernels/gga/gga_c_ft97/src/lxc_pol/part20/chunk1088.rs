//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1088/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1088<F: Float>(t13571: F, t200: F, t679: F, t3750: F, t3773: F, t6027: F, t108576: F, t108583: F, t108586: F, t108587: F, t108591: F, t108593: F, t108597: F, t108600: F, t108606: F, t108618: F, t13395: F, t13402: F, t13651: F, t17819: F, t17859: F, t231: F, t24315: F, t24361: F, t24363: F, t27500: F, t27506: F, t27584: F, t27642: F, t3774: F, t6023: F, t6043: F, t6045: F, t684: F, t96451: F, t98545: F) -> (F, F) {
    let t108624 = t679 * t13571 * t200;
    let t108629 = t3773 * t6027 * t3750;
    let t108632 = t108576 - 0.76612330055555555555e-1 * t96451 * t98545 * t17859 * t684 - 0.39591381038172075259e-3 * t108583 - 0.10357803499222879255e-4 * t108586 * t108587 - 0.30697322007724579004e-7 * t108591 * t108593 + 0.85124811172839506173e-2 * t108597 - 0.38306165027777777778e-1 * t27500 * t108600 - 0.6809984893827160494e-1 * t24361 * t27642 * t24363 + 0.91830411319857336049e-5 * t3774 * t108606 * t13402 + 0.38306165027777777778e-1 * t6043 * t6045 * t231 * t13651 + 0.27568129967481981592e-4 * t3774 * t27584 * t13395 - 0.85124811172839506173e-2 * t108618 - 0.10214977340740740741e0 * t6043 * t27506 * t24315 + 0.25845121844514357744e-4 * t3774 * t6023 * t108624 - 0.12020514968855939808e-5 * t17819 * t108629;
    (t108624, t108632)
}
