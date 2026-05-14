//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1117/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1117<F: Float>(t10709: F, t25227: F, t2661: F, t240: F, t25260: F, t10728: F, t2479: F, t25222: F, t25228: F, t9775: F, t10732: F, t10700: F, t7045: F, t10705: F, t25234: F, t93058: F, t93063: F, t93067: F, t93069: F, t93073: F, t93075: F, t93077: F) -> (F,) {
    let t93080 = t2661 * t25227 * t10709;
    let t93082 = t25260 * t240;
    let t93084 = t2661 * t93082 * t10728;
    let t93086 = t25222 * t2479;
    let t93088 = t9775 * t25228;
    let t93091 = t2661 * t25227 * t10732;
    let t93093 = t7045 * t10700;
    let t93095 = t25234 * t10705;
    let t93097 = -0.76230004213927992339e-4 * t93058 - 0.25724410870841842183e-2 * t93063 - 0.13605355082800796533e0 * t93067 + 0.24009450146119052704e-1 * t93069 + 0.32524801797942610064e-2 * t93073 - 0.17149607247227894789e-2 * t93075 - 0.30492001685571196935e-3 * t93077 + 0.42874018118069736972e-4 * t93080 - 0.85748036236139473944e-4 * t93084 - 0.12004725073059526352e0 * t93086 - 0.45732285992607719437e-3 * t93088 + 0.42874018118069736972e-4 * t93091 - 0.51448821741683684367e-1 * t93093 + 0.15246000842785598468e-2 * t93095;
    (t93097,)
}
