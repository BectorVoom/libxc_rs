//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 933/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk933(t1330: f64, t21134: f64, t1451: f64, t21106: f64, t1430: f64, t21125: f64, t21110: f64, t21073: f64, t21078: f64, t542: f64, t104: f64, t111: f64, t120: f64, t12049: f64, t17150: f64, t17151: f64, t21721: f64, t21723: f64, t21725: f64, t21727: f64, t21729: f64, t21731: f64, t21734: f64, t21737: f64, t4865: f64, t4881: f64) -> f64 {
    let t21740 = t1330 * t21134;
    let t21743 = t1451 * t21106;
    let t21746 = t1430 * t21125;
    let t21749 = t1451 * t21110;
    let t21752 = t1430 * t21073;
    let t21755 = t542 * t21078;
    let t21758 = t542 * t21125;
    let t21761 = 0.15684083333333333333e-4_f64 * t21721 - 0.9368e-2_f64 * t21723 - 0.13208333333333333333e-2_f64 * t21725 + 0.88055555555555555555e-3_f64 * t21727 - 0.117630625e-4_f64 * t21729 + 0.4684e-2_f64 * t21731 - t17150 - 0.31226666666666666667e-2_f64 * t17151 - t12049 + 0.317e-2_f64 * t111 * t21734 - 0.17611111111111111111e-3_f64 * t111 * t21737 + 0.21133333333333333333e-2_f64 * t4865 * t21740 + 0.30247875e-4_f64 * t120 * t21743 + 0.403305e-4_f64 * t120 * t21746 + 0.403305e-4_f64 * t4881 * t21749 + 0.7026e-2_f64 * t104 * t21752 + 0.1171e-2_f64 * t104 * t21755 - 0.7026e-2_f64 * t104 * t21758;
    t21761
}
