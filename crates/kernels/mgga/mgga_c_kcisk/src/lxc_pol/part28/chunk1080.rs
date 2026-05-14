//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1080/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1080<F: Float>(t4790: F, t8607: F, t1683: F, t6874: F, t7509: F, t10699: F, t8590: F, t1973: F, t9109: F, t2605: F, t7489: F, t9128: F, t9125: F, t5400: F, t9124: F, t7493: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24747 = t8607 * t4790;
    let t24748 = t24747 * t1683;
    let t24751 = t7509 * t6874;
    let t24754 = t8590 * t10699;
    let t24755 = t24754 * t1683;
    let t24762 = t9109 * t1973;
    let t24765 = t2605 * t7489;
    let t24768 = t9128 * t1973;
    let t24771 = t9125 * t1973;
    let t24774 = t9124 * t5400;
    let t24775 = t24774 * t1973;
    let t24778 = t7493 * t7489;
    (t24748, t24751, t24755, t24762, t24765, t24768, t24771, t24775, t24778)
}
