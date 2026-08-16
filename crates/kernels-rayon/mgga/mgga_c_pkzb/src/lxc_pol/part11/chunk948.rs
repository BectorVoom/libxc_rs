//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 948/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk948(t10361: f64, t942: f64, t10283: f64, t10297: f64, t10300: f64, t10306: f64, t1246: f64, t1256: f64, t3247: f64, t3255: f64, t3279: f64, t3904: f64, t3910: f64, t3929: f64, t411: f64, t415: f64, t938: f64, t952: f64) -> (f64, f64) {
    let t10362 = t942 * t10361;
    let t10365 = 0.65854491829355115987e0_f64 * t10283 * t415 - 0.65854491829355115987e0_f64 * t3904 * t952 - 0.13170898365871023197e1_f64 * t3247 * t1256 + 0.26341796731742046394e1_f64 * t1246 * t3255 - 0.13170898365871023197e1_f64 * t1246 * t3279 + 0.13170898365871023197e1_f64 * t938 * t3910 - 0.39512695097613069591e1_f64 * t411 * t10297 + 0.26341796731742046394e1_f64 * t411 * t10300 - 0.65854491829355115987e0_f64 * t938 * t3929 + 0.13170898365871023197e1_f64 * t411 * t10306 - 0.65854491829355115987e0_f64 * t411 * t10362;
    (t10362, t10365)
}
