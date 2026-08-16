//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 833/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk833(t188: f64, t41762: f64, t3153: f64, t8063: f64, t12914: f64, t1562: f64, t4614: f64, t12806: f64, t4540: f64, t4673: f64, t3116: f64, t7995: f64) -> (f64, f64, f64, f64, f64) {
    let t41763 = t188 * t41762;
    let t41767 = 0.23833659967900284446e0_f64 * t3153 * t8063;
    let t41769 = t1562 * t4614 * t12914;
    let t41773 = 0.14300195980740170667e1_f64 * t4540 * t4673 * t12806;
    let t41774 = t7995 * t3116;
    (t41763, t41767, t41769, t41773, t41774)
}
