//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 875/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk875<F: Float>(t13620: F, t2087: F, t4614: F, t13631: F, t825: F, t826: F, t2684: F, t7354: F, t45423: F, t7427: F, t7573: F, t10915: F, t22242: F, t45316: F) -> (F, F, F, F, F) {
    let t45611 = F::new(0.92023022289409799224e1) * t2087 * t4614 * t13620;
    let t45613 = t825 * t826 * t13631;
    let t45614 = F::new(0.25561950635947166451e0) * t45613;
    let t45616 = t2684 * t7354 * t13631;
    let t45617 = F::new(0.25561950635947166451e0) * t45616;
    let t45627 = F::new(0.62115540045351614476e2) * t7427 * t7573 * t45423;
    let t45630 = F::new(0.21450293971110256001e1) * t22242 * t10915 * t45316;
    (t45611, t45614, t45617, t45627, t45630)
}
