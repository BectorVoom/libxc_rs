//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1198/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1198<F: Float>(t663: F, t7502: F, t1898: F, t2743: F, t713: F, t7510: F, t694: F, t7518: F, t237: F, t5845: F, t307: F, t6000: F, t2887: F, t2890: F, t487: F, t68: F, t7593: F) -> (F, F, F, F, F, F, F, F) {
    let t21215 = t7502 * t663;
    let t21221 = t2743 * t1898;
    let t21226 = t7510 * t713;
    let t21229 = t7518 * t694;
    let t21267 = t237 * t5845;
    let t21346 = t307 * t6000;
    let t21359 = t2887 * t487 * t2890;
    let t21362 = t2887 * t68 * t7593;
    (t21215, t21221, t21226, t21229, t21267, t21346, t21359, t21362)
}
