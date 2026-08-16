//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3236/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3236(t18662: f64, t41070: f64, t686: f64, t72: f64, t18658: f64, t786: f64, t789: f64, t18796: f64, t2465: f64, t2470: f64, t15011: f64, t18800: f64, t2770: f64, t2772: f64, t39549: f64, t39550: f64, t4487: f64, t50155: f64, t50164: f64, t50166: f64, t50169: f64, t50174: f64, t50178: f64, t50183: f64, t61324: f64, t61326: f64, t61330: f64, t61337: f64, t61344: f64, t865: f64) -> f64 {
    let t61348 = t41070 * t18662 * t72 * t686;
    let t61351 = t786 * t18658 * t789;
    let t61355 = t2465 * t18796 * t2470;
    let t61358 = -0.22089088168956307394e-3_f64 * t50155 - 0.65049603595885220126e-3_f64 * t61324 + 0.26341796731742046394e1_f64 * t865 * t2770 * t61326 - 0.13009920719177044025e-1_f64 * t61330 + 0.65854491829355115984e-1_f64 * t50164 - 0.34146773541147097178e-1_f64 * t50166 - 0.21951497276451705328e-1_f64 * t50169 + 0.10975748638225852664e-1_f64 * t50174 - 0.39274398764404314548e-3_f64 * t50178 - 0.14634331517634470219e-1_f64 * t61337 + 0.52683593463484092788e1_f64 * t15011 * t4487 + 0.13170898365871023197e1_f64 * t18800 * t2772 + 0.21951497276451705328e-1_f64 * t61344 - 0.11708928647259339622e0_f64 * t61348 - t39549 + 0.19514881078765566038e-1_f64 * t61351 - 0.22089088168956307394e-3_f64 * t39550 + 0.13009920719177044025e-1_f64 * t61355 - 0.29268663035268940438e-1_f64 * t50183;
    t61358
}
