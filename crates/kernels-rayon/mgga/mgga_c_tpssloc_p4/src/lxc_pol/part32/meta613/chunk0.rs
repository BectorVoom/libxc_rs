//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2013/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2013(t81954: f64, t244: f64, t6546: f64, t1878: f64, t845: f64, t2230: f64, t23076: f64, t213: f64, t200: f64, t23075: f64, t598: f64, t23034: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t81955 = 0.33643963411783659044e-4_f64 * t81954;
    let t81956 = t6546 * t244;
    let t81959 = t1878 * t845;
    let t81962 = t2230 * t23076;
    let t81963 = t81962 * t213;
    let t81968 = t598 / t23075 / t200;
    let t81979 = t6546 * t23034;
    (t81955, t81956, t81959, t81962, t81963, t81968, t81979)
}
