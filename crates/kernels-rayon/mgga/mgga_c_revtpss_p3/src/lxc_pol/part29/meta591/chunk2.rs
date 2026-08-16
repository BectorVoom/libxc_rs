//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1965/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1965(t2435: f64, t28902: f64, t7515: f64, t98308: f64, t97962: f64, t14110: f64, t96463: f64, t5775: f64, t689: f64, t7492: f64, t2453: f64, t3908: f64, t8086: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102249 = t2435 * t28902;
    let t102253 = 0.14456046980341999104e-1_f64 * t98308 * t7515;
    let t102255 = 0.25702851531048074406e-1_f64 * t97962 * t7515;
    let t102257 = t96463 * t14110;
    let t102261 = 0.10975748638225852664e-1_f64 * t689 * t7492 * t5775;
    let t102266 = t2453 * t8086 * t3908;
    (t102249, t102253, t102255, t102257, t102261, t102266)
}
