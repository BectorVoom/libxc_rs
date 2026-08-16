//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 847/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk847(t6883: f64, t8631: f64, t2085: f64, t552: f64, t794: f64, t8630: f64, t6897: f64, t1338: f64, t8617: f64, t8622: f64, t225: f64, t8618: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31616 = t6883 * t8631;
    let t31617 = 0.19190897446562641759e-1_f64 * t31616;
    let t31618 = t552 * t2085;
    let t31623 = t794 * t8630;
    let t31624 = t6897 * t31623;
    let t31625 = 0.41123351671205660912e-2_f64 * t31624;
    let t31636 = t1338 * t8617;
    let t31648 = t6883 * t8622;
    let t31649 = 0.19190897446562641759e-1_f64 * t31648;
    let t31653 = t8618 * t225;
    (t31617, t31618, t31623, t31625, t31636, t31649, t31653)
}
