//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1249/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1249(t3981: f64, t94443: f64, t7271: f64, t9944: f64, t25986: f64, t2661: f64, t9930: f64, t9757: f64, t94418: f64, t94420: f64, t94424: f64, t94426: f64, t94430: f64, t94432: f64, t94434: f64, t94436: f64, t94438: f64, t94440: f64) -> f64 {
    let t94444 = t94443 * t3981;
    let t94446 = t7271 * t9944;
    let t94449 = t2661 * t25986 * t9930;
    let t94451 = t7271 * t9757;
    let t94453 = 0.51448821741683684367e-2_f64 * t94418 + 0.51448821741683684367e-2_f64 * t94420 + 0.6098400337114239387e-3_f64 * t94424 - 0.25724410870841842184e-1_f64 * t94426 - 0.48018900292238105409e-1_f64 * t94430 - 0.10289764348336736873e-1_f64 * t94432 + 0.25724410870841842183e-2_f64 * t94434 - 0.12862205435420921092e-2_f64 * t94436 + 0.51448821741683684367e-2_f64 * t94438 + 0.25724410870841842183e-2_f64 * t94440 + 0.32524801797942610064e-2_f64 * t94444 - 0.51448821741683684367e-1_f64 * t94446 + 0.42874018118069736972e-4_f64 * t94449 - 0.17149607247227894789e-2_f64 * t94451;
    t94453
}
