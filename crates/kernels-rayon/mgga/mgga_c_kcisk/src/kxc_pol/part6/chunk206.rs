//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 206/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk206(t829: f64, t830: f64, t815: f64, t2: f64, t45: f64, t142: f64, t56: f64, t69: f64, t47: f64, t52: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t831 = t829 * t830;
    let t833 = 1.0_f64 * t815 * t831;
    let t834 = t45 * t2;
    let t836 = t69 * t142 * t56;
    let t839 = t45 * t47;
    let t840 = t52 * t52;
    let t841 = 1.0_f64 / t840;
    (t831, t833, t834, t836, t839, t840, t841)
}
