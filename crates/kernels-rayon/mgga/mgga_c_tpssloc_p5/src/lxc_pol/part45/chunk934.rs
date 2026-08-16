//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 934/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk934(t1880: f64, t23237: f64, t30656: f64, t1888: f64, t23270: f64, t2742: f64, t30633: f64, t112660: f64, t6552: f64, t6555: f64, t23030: f64, t30638: f64) -> (f64, f64, f64, f64) {
    let t112920 = 0.3289868133696452873e-1_f64 * t1880 * t23237 * t30656;
    let t112927 = 0.3289868133696452873e-1_f64 * t1888 * t23270 * t30633 * t2742;
    let t112932 = 0.6579736267392905746e-1_f64 * t6552 * t112660 * t6555;
    let t112936 = 0.52089578783527170489e-1_f64 * t23030 * t30638;
    (t112920, t112927, t112932, t112936)
}
