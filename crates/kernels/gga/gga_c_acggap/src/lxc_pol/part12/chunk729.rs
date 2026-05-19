//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 729/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk729<F: Float>(t7760: F, t1426: F, t2085: F, t429: F, t598: F, t368: F, t7470: F, t7476: F, t7483: F, t1980: F, t1967: F, t1973: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7761 = F::cast_from(0.10718504529517434243e-2_f64) * t7760;
    let t7763 = t1426 * t429 * t2085;
    let t7764 = t598 * t7763;
    let t7767 = t1426 * t368 * t7470;
    let t7768 = t598 * t7767;
    let t7770 = t7476 * t7483;
    let t7771 = t1980 * t7770;
    let t7772 = F::cast_from(0.7145669686344956162e-3_f64) * t7771;
    let t7773 = t1967 * t1973;
    (t7761, t7763, t7764, t7767, t7768, t7770, t7771, t7772, t7773)
}
