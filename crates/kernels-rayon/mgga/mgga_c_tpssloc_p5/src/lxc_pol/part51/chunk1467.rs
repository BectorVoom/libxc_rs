//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1467/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1467(t12524: f64, t33656: f64, t27254: f64, t6534: f64, t120833: f64, t8657: f64, t31814: f64, t33185: f64, t31817: f64, t1873: f64, t94127: f64, t120849: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t122776 = 27.0_f64 * t12524 * t33656;
    let t122780 = 0.135e2_f64 * t27254 * t6534;
    let t122784 = 27.0_f64 * t120833 * t8657;
    let t122786 = 27.0_f64 * t33185 * t31814;
    let t122788 = 27.0_f64 * t33185 * t31817;
    let t122790 = 0.135e2_f64 * t94127 * t1873;
    let t122794 = 27.0_f64 * t120849 * t8657;
    (t122776, t122780, t122784, t122786, t122788, t122790, t122794)
}
