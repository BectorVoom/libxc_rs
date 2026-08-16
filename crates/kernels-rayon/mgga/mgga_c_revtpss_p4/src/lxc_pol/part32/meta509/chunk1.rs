//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1800/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1800(t30247: f64, t545: f64, t2028: f64, t2097: f64, t6918: f64, t7296: f64, t2027: f64, t2103: f64, t26309: f64, t26361: f64, t26363: f64, t26365: f64, t27837: f64, t28826: f64, t28838: f64, t28846: f64, t28853: f64, t28858: f64, t28895: f64, t28897: f64, t28903: f64, t28909: f64, t30071: f64, t6919: f64, t7295: f64, t7511: f64, t7917: f64, t8095: f64, t8104: f64) -> (f64, f64, f64, f64, f64) {
    let t30295 = t545 * t30247;
    let t30296 = t2028 * t30295;
    let t30308 = t2097 * t6918;
    let t30309 = t7296 * t30308;
    let t30312 = 0.17347256376410398924e1_f64 * t27837 * t8095 + t26309 - 0.10975748638225852664e-1_f64 * t28826 + 0.19514881078765566038e-1_f64 * t28838 - 0.65854491829355115987e0_f64 * t7511 * t6919 + 0.14456046980341999104e-1_f64 * t28846 - 0.19514881078765566038e-1_f64 * t28853 - 0.4336814094102599731e0_f64 * t2027 * t30296 - 0.25702851531048074406e-1_f64 * t28858 - t26361 + t26363 - 0.8673628188205199462e0_f64 * t7917 * t8104 - t26365 - 0.14456046980341999104e-1_f64 * t28895 + 0.25702851531048074406e-1_f64 * t28897 - 0.4336814094102599731e0_f64 * t30071 * t2103 + 0.10975748638225852664e-1_f64 * t28903 + 0.14456046980341999104e-1_f64 * t28909 + 0.8673628188205199462e0_f64 * t7295 * t30309;
    (t30295, t30296, t30308, t30309, t30312)
}
