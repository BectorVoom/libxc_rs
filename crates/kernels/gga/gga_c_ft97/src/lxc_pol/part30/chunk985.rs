//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 985/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk985<F: Float>(t230: F, t4125: F, t420: F, t7470: F, t150709: F, t35955: F, t111838: F, t127655: F, t127659: F, t150547: F, t150758: F, t150846: F, t153011: F, t153044: F, t153130: F, t153133: F, t153136: F, t153143: F, t153147: F, t153155: F, t153164: F, t19116: F, t28652: F, t28677: F, t28680: F, t291: F, t31465: F, t33948: F, t35879: F, t35917: F, t35961: F, t70497: F, t70550: F, t70779: F, t7607: F) -> (F, F) {
    let t153169 = t7470 * t420 * t230 * t4125;
    let t153181 = t35955 * t150709;
    let t153183 = -0.19592980390298668092e-1 * t153130 * t150846 + 0.19592980390298668092e-1 * t153133 * t150846 + 0.70628613596813898777e-2 * t153136 * t150758 + 0.82108427773942439976e0 * t70779 * t153044 - 0.41054213886971219988e0 * t70550 * t153143 + 0.41054213886971219988e0 * t7607 * t153147 + 0.82108427773942439976e0 * t70779 * t153143 - 0.82108427773942439976e0 * t70497 * t153147 + 0.22653425206514361674e0 * t31465 * t153155 + 0.6041940442683716741e-1 * t111838 * t153011 - 0.18125821328051150223e0 * t127659 * t35879 - 0.18125821328051150223e0 * t28677 * t153164 + 0.18125821328051150223e0 * t28680 * t153169 + 0.18125821328051150223e0 * t28652 * t153164 + 0.45306850413028723348e0 * t19116 * t291 * t35961 - 0.21188584079044169634e-1 * t35917 * t127655 - 0.14227058655052092711e0 * t33948 * t150547 + 0.19592980390298668092e-1 * t153181;
    (t153169, t153183)
}
