//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 848/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk848(t10318: f64, t1397: f64, t9287: f64, t2487: f64, t2754: f64, t9438: f64, t9448: f64, t204: f64, t2476: f64, t41810: f64, t1445: f64, t1562: f64, t2854: f64, t9127: f64) -> (f64, f64, f64, f64) {
    let t41914 = t1397 * t10318 * t9287;
    let t41915 = 0.29792074959875355558e-1_f64 * t41914;
    let t41918 = t2487 * t9438 * t9448 * t2754;
    let t41919 = 0.15976219147466979032e-1_f64 * t41918;
    let t41922 = 0.46011511144704899612e1_f64 * t2476 * t204 * t41810;
    let t41927 = 0.69017266717057349418e1_f64 * t1562 * t1445 * t2854 * t9127;
    (t41915, t41919, t41922, t41927)
}
