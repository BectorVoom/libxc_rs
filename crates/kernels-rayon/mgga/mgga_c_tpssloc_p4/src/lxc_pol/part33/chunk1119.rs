//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1119/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1119(t23122: f64, t25064: f64, t4166: f64, t6620: f64, t1516: f64, t23133: f64, t7503: f64, t838: f64, t23046: f64, t242: f64, t812: f64, t23062: f64, t7497: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25065 = t23122 * t25064;
    let t25068 = t4166 * t6620;
    let t25077 = t23133 * t1516;
    let t25080 = t7503 * t838;
    let t25083 = t23046 * t242;
    let t25084 = t812 * t25083;
    let t25109 = t23062 * t7497;
    (t25065, t25068, t25077, t25080, t25084, t25109)
}
