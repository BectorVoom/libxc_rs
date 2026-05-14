//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 687/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk687<F: Float>(t1426: F, t368: F, t7470: F, t598: F, t7476: F, t7483: F, t1980: F, t1967: F, t1973: F, t1985: F, t7637: F, t7508: F, t56: F, t593: F, t151: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7767 = t1426 * t368 * t7470;
    let t7768 = t598 * t7767;
    let t7770 = t7476 * t7483;
    let t7771 = t1980 * t7770;
    let t7772 = 0.7145669686344956162e-3 * t7771;
    let t7773 = t1967 * t1973;
    let t7774 = 0.12862205435420921092e-2 * t7773;
    let t7775 = t7637 * t1985;
    let t7776 = 0.95275595817932748827e-3 * t7775;
    let t7777 = 1.0 / t7508;
    let t7778 = t7777 * t56;
    let t7779 = t593 * t7778;
    let t7780 = t151 * t7779;
    (t7767, t7768, t7770, t7771, t7772, t7773, t7774, t7776, t7777, t7779, t7780)
}
