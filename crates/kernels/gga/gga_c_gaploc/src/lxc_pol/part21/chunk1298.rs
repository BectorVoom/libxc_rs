//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1298/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1298<F: Float>(t8158: F, t9333: F, t1508: F, t2765: F, t3377: F, t524: F, t7930: F, t8155: F, t1572: F, t16251: F, t3354: F, t14626: F, t597: F) -> (F, F, F, F, F, F) {
    let t34145 = F::cast_from(0.21450293971110256002e1_f64) * t8158 * t9333;
    let t34148 = F::cast_from(0.10725146985555128001e1_f64) * t1508 * t2765 * t3377;
    let t34151 = F::cast_from(0.21450293971110256002e1_f64) * t524 * t7930 * t3377;
    let t34153 = F::cast_from(0.21450293971110256002e1_f64) * t8155 * t9333;
    let t34156 = F::cast_from(0.15889106645266856297e0_f64) * t1572 * t16251 * t3354;
    let t34178 = F::cast_from(0.51123901271894332903e1_f64) * t597 * t14626 * t3354;
    (t34145, t34148, t34151, t34153, t34156, t34178)
}
