//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 933/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk933<F: Float>(t1330: F, t21134: F, t1451: F, t21106: F, t1430: F, t21125: F, t21110: F, t21073: F, t21078: F, t542: F, t104: F, t111: F, t120: F, t12049: F, t17150: F, t17151: F, t21721: F, t21723: F, t21725: F, t21727: F, t21729: F, t21731: F, t21734: F, t21737: F, t4865: F, t4881: F) -> F {
    let t21740 = t1330 * t21134;
    let t21743 = t1451 * t21106;
    let t21746 = t1430 * t21125;
    let t21749 = t1451 * t21110;
    let t21752 = t1430 * t21073;
    let t21755 = t542 * t21078;
    let t21758 = t542 * t21125;
    let t21761 = F::cast_from(0.15684083333333333333e-4_f64) * t21721 - F::new(0.9368e-2) * t21723 - F::cast_from(0.13208333333333333333e-2_f64) * t21725 + F::cast_from(0.88055555555555555555e-3_f64) * t21727 - F::cast_from(0.117630625e-4_f64) * t21729 + F::new(0.4684e-2) * t21731 - t17150 - F::cast_from(0.31226666666666666667e-2_f64) * t17151 - t12049 + F::new(0.317e-2) * t111 * t21734 - F::cast_from(0.17611111111111111111e-3_f64) * t111 * t21737 + F::cast_from(0.21133333333333333333e-2_f64) * t4865 * t21740 + F::new(0.30247875e-4) * t120 * t21743 + F::new(0.403305e-4) * t120 * t21746 + F::new(0.403305e-4) * t4881 * t21749 + F::new(0.7026e-2) * t104 * t21752 + F::new(0.1171e-2) * t104 * t21755 - F::new(0.7026e-2) * t104 * t21758;
    t21761
}
