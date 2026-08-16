//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1604/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1604(t3362: f64, t3603: f64, t2251: f64, t12773: f64, t12784: f64, t13061: f64, t44173: f64, t10356: f64, t1214: f64, t12772: f64, t12835: f64, t3625: f64) -> (f64, f64, f64, f64, f64) {
    let t44190 = t3603 * t3362;
    let t44191 = t44190 * t2251;
    let t44200 = t12784 * t12773;
    let t44202 = t44173 * t13061;
    let t44205 = t10356 * t1214;
    let t44215 = t3625 * t12772 * t12835;
    (t44191, t44200, t44202, t44205, t44215)
}
