//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 718/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk718(t5331: f64, t555: f64, t1507: f64, t540: f64, t1497: f64, t1503: f64, t1499: f64, t1511: f64, t4952: f64, t534: f64, t541: f64, t133: f64, t1773: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5333 = 0.35089341735807877242e1_f64 * t555 * t5331;
    let t5335 = t1507 * t540;
    let t5336 = t1503 * t1497 * t5335;
    let t5338 = 0.51947577317044391277e2_f64 * t555 * t5336;
    let t5339 = t1511 * t1499;
    let t5342 = t534 * t4952 * t541;
    let t5344 = 0.5848223622634646207e0_f64 * t555 * t5342;
    let t5356 = t133 * t1773;
    (t5333, t5335, t5336, t5338, t5339, t5342, t5344, t5356)
}
