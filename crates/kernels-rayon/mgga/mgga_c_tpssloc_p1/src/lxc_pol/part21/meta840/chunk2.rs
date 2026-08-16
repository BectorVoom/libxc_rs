//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3015/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3015(t1049: f64, t1058: f64, t1060: f64, t11065: f64, t14488: f64, t14578: f64, t14606: f64, t14622: f64, t14640: f64, t14645: f64, t1610: f64, t1625: f64, t1630: f64, t17959: f64, t18080: f64, t18103: f64, t18161: f64, t3120: f64, t3200: f64, t381: f64, t4649: f64, t4657: f64, t4669: f64, t4684: f64, t47841: f64, t50535: f64, t5914: f64, t5932: f64, t62757: f64) -> f64 {
    let t63133 = 2.0_f64 * t1049 * t1058 * t1060 * t17959 + 2.0_f64 * t1058 * t1060 * t14488 * t1625 + t1058 * t1060 * t3120 * t5914 + t1058 * t1060 * t381 * t62757 + 4.0_f64 * t1058 * t1060 * t4649 * t4657 - 12.0_f64 * t11065 * t18080 * t18103 - 2.0_f64 * t14622 * t3200 * t5932 - 2.0_f64 * t18161 * t3200 * t4684 + 12.0_f64 * t14578 * t47841 + 2.0_f64 * t14606 * t4669 + 2.0_f64 * t14640 * t1610 + 4.0_f64 * t14645 * t4669 + 2.0_f64 * t1630 * t50535;
    t63133
}
