//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2570/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2570(t11262: f64, t1247: f64, t5286: f64, t13099: f64, t43776: f64, t12909: f64, t17395: f64, t44546: f64, t5331: f64, t5334: f64, t13032: f64, t17528: f64) -> (f64, f64, f64, f64, f64) {
    let t57125 = t1247 * t11262 * t5286;
    let t57126 = 0.14291339372689912324e-3_f64 * t57125;
    let t57136 = t13099 * t43776;
    let t57147 = t12909 * t17395;
    let t57222 = t5331 * t44546 * t5334;
    let t57223 = 0.14291339372689912324e-3_f64 * t57222;
    let t57229 = t13032 * t17528;
    (t57126, t57136, t57147, t57223, t57229)
}
