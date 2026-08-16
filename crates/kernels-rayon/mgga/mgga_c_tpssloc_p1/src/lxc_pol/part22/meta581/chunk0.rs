//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2090/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2090(t11789: f64, t820: f64, t11737: f64, t44857: f64, t11647: f64, t1203: f64, t204: f64, t486: f64, t1213: f64, t1216: f64, t248: f64, t11716: f64, t44833: f64, t44834: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44951 = t820 * t11789;
    let t44965 = t44857 * t11737;
    let t45002 = t1203 * t11647;
    let t45017 = t204 * t486;
    let t45020 = t1213 * t248 * t45017 * t1216;
    let t45030 = t44833 * t11716 * t44834;
    (t44951, t44965, t45002, t45017, t45020, t45030)
}
