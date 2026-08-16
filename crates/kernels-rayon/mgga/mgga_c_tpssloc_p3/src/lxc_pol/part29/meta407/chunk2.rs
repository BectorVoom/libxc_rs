//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1659/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1659(t15806: f64, t15833: f64, t11947: f64, t1763: f64, t1256: f64, t14963: f64, t14969: f64, t14971: f64, t15038: f64, t15040: f64, t15043: f64, t15046: f64, t15048: f64, t15050: f64, t15053: f64, t15056: f64, t15059: f64, t15063: f64, t15066: f64, t15070: f64, t15235: f64, t15237: f64, t193: f64, t336: f64, t3633: f64, t3637: f64, t4700: f64, t5095: f64) -> (f64, f64) {
    let t15834 = t15806 + t15833;
    let t15838 = t1763 * t11947;
    let t15842 = t1256 * t15834 * t193 * t336 + 2.0_f64 * t15838 * t3637 * t4700 - t3633 * t4700 * t5095 + t14963 - t14969 - t14971 - t15038 - t15040 - t15043 + t15046 - t15048 + t15050 - t15053 - t15056 - t15059 + t15063 + t15066 + t15070 + t15235 + t15237;
    (t15834, t15842)
}
