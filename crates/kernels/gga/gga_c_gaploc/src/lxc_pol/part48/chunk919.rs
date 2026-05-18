//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 919/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk919<F: Float>(t13620: F, t2087: F, t4614: F, t13631: F, t825: F, t826: F, t2684: F, t7354: F, t10930: F, t10931: F, t13672: F, t2009: F, t41231: F, t43756: F, t43758: F, t43760: F, t45350: F, t45575: F, t45577: F, t45580: F, t45586: F, t45588: F, t45598: F, t45600: F, t45603: F, t45606: F, t45608: F, t773: F) -> F {
    let t45611 = F::new(0.92023022289409799224e1) * t2087 * t4614 * t13620;
    let t45613 = t825 * t826 * t13631;
    let t45614 = F::new(0.25561950635947166451e0) * t45613;
    let t45616 = t2684 * t7354 * t13631;
    let t45617 = F::new(0.25561950635947166451e0) * t45616;
    let t45619 = t45575 + t45577 - t45580 + F::new(0.55213813373645879536e2) * t10930 * t10931 * t45350 + t45586 - t45588 - F::new(0.35750489951850426669e0) * t773 * t13672 * t2009 + F::new(0.38342925953920749677e1) * t43756 - F::new(0.51123901271894332903e1) * t43758 + F::new(0.38342925953920749677e1) * t43760 + t45598 + t45600 + t45603 + t45606 - t45608 - t45611 + t45614 - t45617 + F::new(0.63904876589867916126e-1) * t41231;
    t45619
}
