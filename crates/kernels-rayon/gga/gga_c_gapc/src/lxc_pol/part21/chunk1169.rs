//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1169/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1169(t20774: f64, t26312: f64, t2993: f64, t26597: f64, t5395: f64, t21072: f64, t27408: f64, t33539: f64, t11308: f64, t11329: f64, t1036: f64, t11488: f64, t21111: f64) -> (f64, f64, f64, f64, f64) {
    let t34477 = t2993 * t26312 * t20774;
    let t34480 = t5395 * t26597 * t20774;
    let t34484 = t21072 * t33539 * t27408;
    let t34486 = t11329 * t11308;
    let t34489 = t11488 * t1036 * t21111;
    (t34477, t34480, t34484, t34486, t34489)
}
