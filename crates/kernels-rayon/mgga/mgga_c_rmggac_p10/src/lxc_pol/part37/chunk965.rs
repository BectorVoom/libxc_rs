//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 965/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk965(t14618: f64, t8368: f64, t14421: f64, t2868: f64, t75119: f64, t75124: f64, t2010: f64, t2265: f64, t8342: f64, t2415: f64, t8048: f64, t8188: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t77457 = t8368 * t14618;
    let t77458 = 0.34093327067806677161e-2_f64 * t77457;
    let t77463 = 0.11974241701863808564e0_f64 * t2868 * t14421;
    let t77464 = 0.1702583995731913576e-4_f64 * t75119;
    let t77465 = 0.85129199786595678799e-5_f64 * t75124;
    let t77467 = t2010 * t8342 * t2265;
    let t77468 = 0.36021158228745895953e-3_f64 * t77467;
    let t77470 = t2010 * t2415 * t8048;
    let t77471 = 0.36021158228745895953e-3_f64 * t77470;
    let t77473 = t2010 * t2415 * t8188;
    (t77458, t77463, t77464, t77465, t77468, t77471, t77473)
}
