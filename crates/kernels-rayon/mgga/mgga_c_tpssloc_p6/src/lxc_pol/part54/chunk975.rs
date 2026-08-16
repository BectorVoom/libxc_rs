//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 975/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk975(t23122: f64, t25064: f64, t4166: f64, t6620: f64, t849: f64, t1516: f64, t23127: f64, t4261: f64, t6621: f64, t23133: f64, t7503: f64, t838: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25065 = t23122 * t25064;
    let t25068 = t4166 * t6620;
    let t25069 = t25068 * t849;
    let t25071 = t23127 * t1516;
    let t25073 = t6621 * t4261;
    let t25077 = t23133 * t1516;
    let t25080 = t7503 * t838;
    (t25065, t25069, t25071, t25073, t25077, t25080)
}
