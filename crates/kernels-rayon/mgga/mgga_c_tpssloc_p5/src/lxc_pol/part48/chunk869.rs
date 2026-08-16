//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 869/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk869(t1378: f64, t31641: f64, t31611: f64, t6891: f64, t6888: f64, t6883: f64, t8622: f64, t22666: f64, t8621: f64, t1985: f64, t225: f64, t8618: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31642 = t1378 * t31641;
    let t31645 = t31611 * t6891;
    let t31646 = t6888 * t31645;
    let t31648 = t6883 * t8622;
    let t31649 = 0.19190897446562641759e-1_f64 * t31648;
    let t31650 = t22666 * t8621;
    let t31651 = t1985 * t31650;
    let t31653 = t8618 * t225;
    (t31642, t31645, t31646, t31649, t31650, t31651, t31653)
}
