//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 449/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk449(t2022: f64, t3: f64, t1401: f64, t1873: f64, t577: f64, t63: f64, t67: f64) -> (f64, f64, f64) {
    let t2023 = t3 * t2022;
    let t2028 = 0.135e2_f64 * t1401 * t1873;
    let t2029 = 0.45e1_f64 * t2022 * t577 + t2028;
    let t2031 = t63 * t67;
    (t2023, t2029, t2031)
}
