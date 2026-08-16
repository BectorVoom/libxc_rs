//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1285/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1285(t61087: f64, t18005: f64, t6134: f64, t19733: f64, t5570: f64, t30: f64, t31814: f64, t2: f64, t2436: f64, t33: f64, t1497: f64, t1317: f64, t5506: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t63998 = 119.0_f64 / 864.0_f64 * t61087;
    let t64060 = t6134 * t18005;
    let t64135 = t19733 * t5570;
    let t64247 = t31814 * t30;
    let t64300 = t2436 * t2;
    let t64879 = t31814 * t33;
    let t64975 = t2436 * t1497;
    let t65157 = t5506 * t1317;
    (t63998, t64060, t64135, t64247, t64300, t64879, t64975, t65157)
}
