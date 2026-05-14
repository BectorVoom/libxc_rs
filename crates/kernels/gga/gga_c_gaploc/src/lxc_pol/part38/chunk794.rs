//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 794/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk794<F: Float>(t13598: F, t5771: F, t1457: F, t2103: F, t44973: F, t45087: F, t13602: F, t2194: F, t13620: F, t2087: F, t4614: F, t13631: F, t825: F, t826: F, t2684: F, t7354: F) -> (F, F, F, F, F, F, F) {
    let t45600 = 0.71500979903700853338e0 * t5771 * t13598;
    let t45603 = 0.71500979903700853338e0 * t2103 * t1457 * t44973;
    let t45606 = 0.71500979903700853338e0 * t2103 * t1457 * t45087;
    let t45608 = 0.92023022289409799224e1 * t2194 * t13602;
    let t45611 = 0.92023022289409799224e1 * t2087 * t4614 * t13620;
    let t45613 = t825 * t826 * t13631;
    let t45614 = 0.25561950635947166451e0 * t45613;
    let t45616 = t2684 * t7354 * t13631;
    (t45600, t45603, t45606, t45608, t45611, t45614, t45616)
}
