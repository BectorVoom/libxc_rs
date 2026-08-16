//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 944/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk944(t1711: f64, t5555: f64, t64: f64, t8052: f64, t136559: f64, t92353: f64, t32214: f64, t5607: f64, t22623: f64, t6: f64, t7837: f64, t92339: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t136648 = t1711 * t5555;
    let t136656 = t64 * t8052 * t5555;
    let t136666 = t92353 * t136559;
    let t136678 = t32214 * t5607;
    let t136679 = t22623 * t136678;
    let t136684 = t7837 * t92339 * t6;
    (t136648, t136656, t136666, t136678, t136679, t136684)
}
