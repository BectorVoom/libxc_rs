//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 899/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk899<F: Float>(t900: F, t9762: F, t3033: F, t3070: F, t3740: F, t6317: F, t2192: F, t3766: F, t3743: F, t6149: F, t836: F, t3041: F, t3046: F) -> (F, F, F, F, F, F, F) {
    let t9764 = F::cast_from(0.5848223622634646207e0_f64) * t9762 * t900;
    let t9766 = F::new(2.0) * t3033 * t3070;
    let t9768 = F::new(2.0) * t6317 * t3740;
    let t9770 = F::new(1.0) * t2192 * t3766;
    let t9771 = t6149 * t3743;
    let t9772 = t9771 * t836;
    let t9774 = t3041 * t3046;
    (t9764, t9766, t9768, t9770, t9771, t9772, t9774)
}
