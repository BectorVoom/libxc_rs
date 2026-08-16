//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1123/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1123(t6535: f64, t914: f64, t18974: f64, t2970: f64, t2463: f64, t23: f64, t4810: f64, t12973: f64, t1440: f64, t1430: f64, t1436: f64, t440: f64, t4803: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19264 = t914 * t6535;
    let t19308 = t2970 * t18974;
    let t19338 = t2463 * t2463;
    let t19339 = 1.0_f64 / t19338;
    let t19377 = t23 * t4810;
    let t19378 = t12973 * t1440;
    let t19381 = t1430 * t1436;
    let t19384 = t1430 * t1440;
    let t19387 = t4803 * t440;
    (t19264, t19308, t19339, t19377, t19378, t19381, t19384, t19387)
}
