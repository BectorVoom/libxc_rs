//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 463/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk463(t1878: f64, t218: f64, t344: f64, t675: f64, t847: f64, t831: f64) -> (f64, f64, f64, f64) {
    let t2221 = t218 * t1878 * t344;
    let t2222 = 0.13692777777777777778e0_f64 * t2221;
    let t2224 = t218 * t675 * t847;
    let t2238 = t831 * t831;
    (t2221, t2222, t2224, t2238)
}
