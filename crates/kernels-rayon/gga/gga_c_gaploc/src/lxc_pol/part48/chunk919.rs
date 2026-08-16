//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 919/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk919(t13620: f64, t2087: f64, t4614: f64, t13631: f64, t825: f64, t826: f64, t2684: f64, t7354: f64, t10930: f64, t10931: f64, t13672: f64, t2009: f64, t41231: f64, t43756: f64, t43758: f64, t43760: f64, t45350: f64, t45575: f64, t45577: f64, t45580: f64, t45586: f64, t45588: f64, t45598: f64, t45600: f64, t45603: f64, t45606: f64, t45608: f64, t773: f64) -> f64 {
    let t45611 = 0.92023022289409799224e1_f64 * t2087 * t4614 * t13620;
    let t45613 = t825 * t826 * t13631;
    let t45614 = 0.25561950635947166451e0_f64 * t45613;
    let t45616 = t2684 * t7354 * t13631;
    let t45617 = 0.25561950635947166451e0_f64 * t45616;
    let t45619 = t45575 + t45577 - t45580 + 0.55213813373645879536e2_f64 * t10930 * t10931 * t45350 + t45586 - t45588 - 0.35750489951850426669e0_f64 * t773 * t13672 * t2009 + 0.38342925953920749677e1_f64 * t43756 - 0.51123901271894332903e1_f64 * t43758 + 0.38342925953920749677e1_f64 * t43760 + t45598 + t45600 + t45603 + t45606 - t45608 - t45611 + t45614 - t45617 + 0.63904876589867916126e-1_f64 * t41231;
    t45619
}
