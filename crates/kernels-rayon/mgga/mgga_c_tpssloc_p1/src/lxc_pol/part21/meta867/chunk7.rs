//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3171/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3171(t15578: f64, t4889: f64, t11789: f64, t1227: f64, t248: f64, t5979: f64, t19051: f64, t3523: f64, t19080: f64, t3572: f64, t1174: f64, t1177: f64, t11825: f64, t1213: f64, t1214: f64, t15581: f64, t15584: f64, t15587: f64, t475: f64, t6203: f64, t63406: f64, t65330: f64, t65613: f64, t65617: f64, t65619: f64, t65628: f64, t65632: f64) -> f64 {
    let t65637 = t4889 * t15578;
    let t65647 = t1227 * t248 * t11789 * t5979;
    let t65649 = t19051 * t3523;
    let t65651 = t19080 * t3572;
    let t65653 = -t65613 / 1728.0_f64 - t65617 / 3456.0_f64 - t65619 / 3456.0_f64 + 5.0_f64 / 13824.0_f64 * t11825 * t6203 + t1213 * t248 * t1214 * t65330 * t475 / 3072.0_f64 - t65628 / 1944.0_f64 + t65632 / 13824.0_f64 - t1174 * t1177 * t63406 / 12.0_f64 + 2.0_f64 / 81.0_f64 * t65637 + 2.0_f64 / 27.0_f64 * t4889 * t15581 + t4889 * t15584 / 27.0_f64 + t4889 * t15587 / 9.0_f64 + t65647 / 20736.0_f64 - t65649 / 3456.0_f64 - t65651 / 216.0_f64;
    t65653
}
