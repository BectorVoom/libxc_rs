//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3096/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3096<F: Float>(t1732: F, t3433: F, t69591: F, t20644: F, t5104: F, t5068: F, t68792: F, t5109: F, t68952: F, t17092: F, t20641: F, t16840: F, t20645: F) -> (F, F, F, F, F, F) {
    let t81618 = F::cast_from(0.48245938496077605201e2_f64) * t3433 * t69591 * t1732;
    let t81621 = F::cast_from(0.48245938496077605201e2_f64) * t3433 * t20644 * t5104;
    let t81623 = F::new(6.0) * t68792 * t5068;
    let t81625 = F::cast_from(0.48245938496077605201e2_f64) * t68952 * t5109;
    let t81627 = F::new(6.0) * t17092 * t20641;
    let t81629 = F::cast_from(0.48245938496077605201e2_f64) * t16840 * t20645;
    (t81618, t81621, t81623, t81625, t81627, t81629)
}
