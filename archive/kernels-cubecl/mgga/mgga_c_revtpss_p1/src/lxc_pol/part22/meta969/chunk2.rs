//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3236/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3236<F: Float>(t18662: F, t41070: F, t686: F, t72: F, t18658: F, t786: F, t789: F, t18796: F, t2465: F, t2470: F, t15011: F, t18800: F, t2770: F, t2772: F, t39549: F, t39550: F, t4487: F, t50155: F, t50164: F, t50166: F, t50169: F, t50174: F, t50178: F, t50183: F, t61324: F, t61326: F, t61330: F, t61337: F, t61344: F, t865: F) -> F {
    let t61348 = t41070 * t18662 * t72 * t686;
    let t61351 = t786 * t18658 * t789;
    let t61355 = t2465 * t18796 * t2470;
    let t61358 = -F::cast_from(0.22089088168956307394e-3_f64) * t50155 - F::cast_from(0.65049603595885220126e-3_f64) * t61324 + F::cast_from(0.26341796731742046394e1_f64) * t865 * t2770 * t61326 - F::cast_from(0.13009920719177044025e-1_f64) * t61330 + F::cast_from(0.65854491829355115984e-1_f64) * t50164 - F::cast_from(0.34146773541147097178e-1_f64) * t50166 - F::cast_from(0.21951497276451705328e-1_f64) * t50169 + F::cast_from(0.10975748638225852664e-1_f64) * t50174 - F::cast_from(0.39274398764404314548e-3_f64) * t50178 - F::cast_from(0.14634331517634470219e-1_f64) * t61337 + F::cast_from(0.52683593463484092788e1_f64) * t15011 * t4487 + F::cast_from(0.13170898365871023197e1_f64) * t18800 * t2772 + F::cast_from(0.21951497276451705328e-1_f64) * t61344 - F::cast_from(0.11708928647259339622e0_f64) * t61348 - t39549 + F::cast_from(0.19514881078765566038e-1_f64) * t61351 - F::cast_from(0.22089088168956307394e-3_f64) * t39550 + F::cast_from(0.13009920719177044025e-1_f64) * t61355 - F::cast_from(0.29268663035268940438e-1_f64) * t50183;
    t61358
}
