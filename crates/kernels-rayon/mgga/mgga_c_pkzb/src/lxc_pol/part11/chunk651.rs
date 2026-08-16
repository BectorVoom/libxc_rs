//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 651/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk651(t3694: f64, t790: f64, t1134: f64, t1144: f64, t307: f64, t311: f64, t3670: f64, t3676: f64, t1147: f64, t135: f64, t2156: f64, t273: f64, t3521: f64, t3523: f64, t3527: f64, t3553: f64, t3556: f64, t3612: f64, t3614: f64, t3616: f64, t3620: f64, t3624: f64, t3628: f64, t805: f64) -> (f64, f64, f64, f64) {
    let t3695 = t790 * t3694;
    let t3698 = 0.65854491829355115987e0_f64 * t3670 * t311 - 0.13170898365871023197e1_f64 * t1134 * t1144 + 0.13170898365871023197e1_f64 * t307 * t3676 - 0.65854491829355115987e0_f64 * t307 * t3695;
    let t3702 = t1147 * t1147;
    let t3706 = -t135 * t2156 * t273 * t3702 + t135 * t273 * t3698 * t805 - t3521 + t3523 - t3527 + t3553 + t3556 + t3612 + t3614 - t3616 + t3620 - t3624 - t3628;
    (t3695, t3698, t3702, t3706)
}
