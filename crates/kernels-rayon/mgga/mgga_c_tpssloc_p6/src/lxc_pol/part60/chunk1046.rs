//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1046/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1046(t1458: f64, t7786: f64, t1874: f64, t27188: f64, t7461: f64, t28835: f64, t8607: f64, t1873: f64, t7467: f64, t128402: f64, t33234: f64, t28017: f64, t7042: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t128920 = t7786 * t1458;
    let t128922 = 4.0_f64 * t128920 * t1874;
    let t128924 = 4.0_f64 * t27188 * t7461;
    let t128926 = 3.0_f64 * t8607 * t28835;
    let t128928 = 4.0_f64 * t128920 * t1873;
    let t128930 = 4.0_f64 * t27188 * t7467;
    let t128932 = 2.0_f64 * t128402 * t1873;
    let t128934 = 4.0_f64 * t33234 * t7467;
    let t128936 = 2.0_f64 * t7042 * t28017;
    (t128922, t128924, t128926, t128928, t128930, t128932, t128934, t128936)
}
