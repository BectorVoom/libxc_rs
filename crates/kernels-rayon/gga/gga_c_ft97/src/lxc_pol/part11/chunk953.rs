//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 953/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk953(t39749: f64, t39751: f64, t446: f64, t1647: f64, t2075: f64, t1969: f64, t1643: f64, t9049: f64, t39708: f64, t39711: f64, t39715: f64, t39717: f64, t39721: f64, t39723: f64, t39728: f64, t39732: f64, t39737: f64, t39741: f64, t39744: f64, t39747: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39753 = t446 * t39749 * t39751;
    let t39755 = t1647 * t2075;
    let t39757 = t446 * t1969 * t39755;
    let t39759 = t1643 * t2075;
    let t39761 = t446 * t9049 * t39759;
    let t39763 = -12.0_f64 * t39708 + 8.0_f64 * t39711 - 4.0_f64 * t39715 - 4.0_f64 / 3.0_f64 * t39717 + 8.0_f64 * t39721 + 16.0_f64 / 9.0_f64 * t39723 + 40.0_f64 / 27.0_f64 * t39728 + 2.0_f64 * t39732 + 4.0_f64 / 3.0_f64 * t39737 + 4.0_f64 / 3.0_f64 * t39741 + 8.0_f64 / 3.0_f64 * t39744 - 8.0_f64 / 9.0_f64 * t39747 + 8.0_f64 * t39753 - 4.0_f64 * t39757 + 4.0_f64 / 3.0_f64 * t39761;
    (t39753, t39755, t39757, t39759, t39761, t39763)
}
