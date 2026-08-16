//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1077/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1077(t11847: f64, t141: f64, t1515: f64, t2202: f64, t1509: f64, t9267: f64, t2869: f64, t9271: f64, t2868: f64, t4079: f64, t1027: f64, t2877: f64, t4071: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11848 = t141 * t11847;
    let t11850 = t2202 * t1515;
    let t11852 = t9267 * t1509;
    let t11853 = t11852 * t2869;
    let t11856 = t9271 * t1509;
    let t11857 = t11856 * t2869;
    let t11859 = t2868 * t4079;
    let t11860 = t11859 * t1027;
    let t11862 = t4071 * t2877;
    (t11848, t11850, t11853, t11857, t11860, t11862)
}
