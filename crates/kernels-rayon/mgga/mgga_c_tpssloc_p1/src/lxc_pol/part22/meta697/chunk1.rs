//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2280/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2280(t15578: f64, t4889: f64, t11789: f64, t1227: f64, t248: f64, t5979: f64, t19051: f64, t3523: f64, t19080: f64, t3572: f64, t11709: f64, t18356: f64) -> (f64, f64, f64, f64, f64) {
    let t65637 = t4889 * t15578;
    let t65647 = t1227 * t248 * t11789 * t5979;
    let t65649 = t19051 * t3523;
    let t65651 = t19080 * t3572;
    let t65660 = t11709 * t18356;
    (t65637, t65647, t65649, t65651, t65660)
}
