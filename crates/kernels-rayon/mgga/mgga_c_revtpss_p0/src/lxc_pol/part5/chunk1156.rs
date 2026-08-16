//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1156/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1156(t14586: f64, t14786: f64, t14791: f64, t1559: f64, t4433: f64, t14785: f64, t2652: f64, t6030: f64, t10858: f64, t6024: f64, t10816: f64, t10824: f64, t10826: f64, t18456: f64, t18459: f64, t18462: f64, t18466: f64, t18471: f64, t18475: f64, t2745: f64, t4362: f64) -> f64 {
    let t18477 = t14586 * t14786;
    let t18478 = t14791 * t18477;
    let t18481 = t1559 * t4433;
    let t18482 = t14785 * t18481;
    let t18485 = t2652 * t6030;
    let t18487 = t10858 * t6024;
    let t18489 = 0.12862205435420921092e-2_f64 * t4362 * t18456 + 0.10003937560882938627e-2_f64 * t18459 - 0.42874018118069736972e-3_f64 * t2745 * t18462 - 0.21437009059034868486e-3_f64 * t2745 * t18466 - 0.42874018118069736972e-2_f64 * t2745 * t18471 - 0.56688979511669985553e-2_f64 * t10816 - 0.20007875121765877254e-1_f64 * t18475 - 0.34299214494455789578e-2_f64 * t4362 * t18478 - 0.85748036236139473945e-2_f64 * t2745 * t18482 + 0.40015750243531754507e-2_f64 * t18485 - t10824 + t10826 - 0.20007875121765877254e-2_f64 * t18487;
    t18489
}
