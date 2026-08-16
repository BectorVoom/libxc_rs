//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 932/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk932(t2033: f64, t2365: f64, t35451: f64, t11784: f64, t2679: f64, t9800: f64, t1445: f64, t1457: f64, t2004: f64, t3025: f64, t3451: f64, t45091: f64, t45772: f64, t45775: f64, t45778: f64, t45785: f64, t45792: f64, t45793: f64, t45794: f64, t45795: f64, t45798: f64, t45801: f64, t45803: f64, t45806: f64, t45809: f64, t45812: f64, t45817: f64, t4752: f64, t807: f64) -> f64 {
    let t45819 = t2033 * t2365 * t35451;
    let t45820 = 0.44688112439813033337e-1_f64 * t45819;
    let t45822 = t9800 * t11784 * t2679;
    let t45823 = 0.9585731488480187419e0_f64 * t45822;
    let t45824 = -t45772 + t45775 - t45778 + 0.23005755572352449806e1_f64 * t807 * t1445 * t45091 + t45785 + 0.35750489951850426669e0_f64 * t2004 * t1457 * t45091 - t45792 - t45793 - t45794 - t45795 + t45798 + t45801 + t45803 + t45806 + t45809 + t45812 - 0.14300195980740170668e1_f64 * t3025 * t4752 * t3451 - t45817 - t45820 + t45823;
    t45824
}
