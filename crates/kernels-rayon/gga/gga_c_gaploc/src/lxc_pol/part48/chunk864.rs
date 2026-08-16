//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 864/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk864(t13525: f64, t325: f64, t550: f64, t42973: f64, t2581: f64, t1841: f64, t35440: f64, t11657: f64, t2554: f64, t7064: f64, t35385: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44771 = t325 * t13525;
    let t44772 = t550 * t44771;
    let t44776 = 0.1281754371690370714e-2_f64 * t42973;
    let t44777 = t550 * t2581;
    let t44780 = 0.10254034973522965711e-1_f64 * t1841 * t35440 * t44777;
    let t44785 = t7064 * t11657 * t2554;
    let t44786 = 0.32043859292259267849e-3_f64 * t44785;
    let t44787 = t883 * t35385;
    (t44771, t44772, t44776, t44777, t44780, t44786, t44787)
}
