//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1820/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1820(t3108: f64, t4640: f64, t1611: f64, t3047: f64, t3103: f64, t4641: f64, t1040: f64, t4616: f64, t1044: f64, t13611: f64, t248: f64, t1023: f64, t13975: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14077 = t4640 * t3108;
    let t14080 = t1611 * t3047;
    let t14084 = t4641 * t3103 / 2304.0_f64;
    let t14085 = t4616 * t1040;
    let t14093 = t248 * t1044 * t13611;
    let t14098 = t13975 * t1023;
    (t14077, t14080, t14084, t14085, t14093, t14098)
}
