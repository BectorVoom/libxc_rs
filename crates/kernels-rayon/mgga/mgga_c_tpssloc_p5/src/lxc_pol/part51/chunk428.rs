//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 428/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk428(t3: f64, t40: f64, t1933: f64, t225: f64, t344: f64, t364: f64) -> (f64, f64, f64, f64) {
    let t1934 = t3 * t40;
    let t1935 = t1933 * t1934;
    let t1936 = t344 * t225;
    let t1937 = t1936 * t364;
    (t1934, t1935, t1936, t1937)
}
