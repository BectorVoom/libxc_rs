//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2965/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2965(t11672: f64, t15682: f64, t12078: f64, t53552: f64, t16183: f64, t73: f64, t42793: f64, t4892: f64, t4895: f64, t15951: f64, t3127: f64, t3172: f64) -> (f64, f64, f64, f64, f64) {
    let t54014 = t11672 * t15682;
    let t54023 = t12078 * t53552;
    let t54026 = t16183 * t73;
    let t54036 = t4892 * t42793 * t4895;
    let t54037 = 0.28582678745379824648e-3_f64 * t54036;
    let t54039 = t3127 * t3172 * t15951;
    (t54014, t54023, t54026, t54037, t54039)
}
