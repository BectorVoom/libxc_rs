//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1083/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1083<F: Float>(t92254: F, t23004: F, t376: F, t5665: F, t22642: F, t22819: F, t22821: F, t22552: F, t22553: F, t22632: F, t1608: F, t1689: F, t5584: F, t1613: F, t22563: F, t7837: F) -> (F, F, F, F, F, F, F) {
    let t92255 = t92254 / 8.0;
    let t92258 = t5665 * t376 * t23004;
    let t92259 = t92258 / 12.0;
    let t92264 = t22819 * t22642 * t22821;
    let t92275 = t22552 * t22632 * t22553;
    let t92278 = t1608 * t5584 * t1689;
    let t92299 = t7837 * t22563 * t1613;
    (t92255, t92258, t92259, t92264, t92275, t92278, t92299)
}
