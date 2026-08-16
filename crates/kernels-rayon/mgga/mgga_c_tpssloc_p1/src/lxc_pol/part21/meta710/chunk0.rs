//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2544/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2544(t10403: f64, t10422: f64, t14214: f64, t3030: f64, t4552: f64, t3032: f64, t3129: f64, t13998: f64, t2960: f64, t42875: f64, t4338: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49629 = t10403 * t10422 * t14214;
    let t49649 = t4552 * t3030;
    let t49650 = t49649 * t3032;
    let t49651 = t49650 * t3129;
    let t49658 = t2960 * t13998;
    let t49661 = t973 * t42875 * t4338;
    (t49629, t49649, t49650, t49651, t49658, t49661)
}
