//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1468/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1468(t11789: f64, t820: f64, t3577: f64, t3579: f64, t11737: f64, t44857: f64, t11791: f64, t3490: f64, t1227: f64, t248: f64, t3252: f64, t3248: f64) -> (f64, f64, f64, f64, f64) {
    let t44951 = t820 * t11789;
    let t44953 = t3577 * t44951 * t3579;
    let t44965 = t44857 * t11737;
    let t44968 = t3490 * t11791;
    let t44972 = t1227 * t248 * t11789 * t3252;
    let t44976 = t1227 * t248 * t11789 * t3248;
    (t44953, t44965, t44968, t44972, t44976)
}
