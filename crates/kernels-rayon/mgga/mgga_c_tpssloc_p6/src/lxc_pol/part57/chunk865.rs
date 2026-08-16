//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 865/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk865(t7458: f64, t8533: f64, t1873: f64, t7890: f64, t652: f64, t1458: f64, t2035: f64) -> (f64, f64, f64, f64) {
    let t33230 = 2.0_f64 * t7458 * t8533;
    let t33231 = t7890 * t1873;
    let t33233 = 2.0_f64 * t652 * t33231;
    let t33234 = t2035 * t1458;
    (t33230, t33231, t33233, t33234)
}
