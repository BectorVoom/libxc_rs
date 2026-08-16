//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 688/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk688(t12806: f64, t6320: f64, t2268: f64, t3148: f64, t988: f64, t12792: f64, t169: f64, t172: f64, t452: f64, t105: f64, t12764: f64, t12769: f64, t12771: f64, t12774: f64, t12794: f64, t12799: f64, t12802: f64, t12805: f64) -> (f64, f64, f64, f64, f64) {
    let t12807 = t6320 * t12806;
    let t12809 = 0.17073003981405689759e0_f64 * t2268 * t12807;
    let t12810 = t3148 * t988;
    let t12812 = 0.28455006635676149599e-1_f64 * t2268 * t12810;
    let t12814 = t12792 * t169 * t172;
    let t12815 = t452 * t12814;
    let t12818 = 0.1138200265427045984e0_f64 * t12764 + t12769 + 0.23712505529730124666e-2_f64 * t12771 - 0.1707300398140568976e0_f64 * t12774 - 0.28455006635676149599e-1_f64 * t105 * t12794 + t12799 - t12802 + t12805 - t12809 + t12812 + 0.28455006635676149599e-1_f64 * t105 * t12815;
    (t12807, t12810, t12814, t12815, t12818)
}
