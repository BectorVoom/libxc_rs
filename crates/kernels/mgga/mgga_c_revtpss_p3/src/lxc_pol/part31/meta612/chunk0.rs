//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2054/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2054<F: Float>(t30: F, t41154: F, t1957: F, t25392: F, t14495: F, t689: F, t25372: F, t25386: F, t27357: F, t14587: F, t27312: F, t92838: F) -> (F, F, F, F, F, F, F) {
    let t98785 = t41154 * t30;
    let t98799 = t1957 * t25392;
    let t98801 = t14495 * t689;
    let t98803 = F::cast_from(0.14456046980341999104e-1_f64) * t25372 * t98799 * t98801;
    let t98806 = F::cast_from(0.25702851531048074406e-1_f64) * t25386 * t98799 * t98801;
    let t98807 = t1957 * t27357;
    let t98809 = t14587 * t689;
    let t98811 = F::cast_from(0.28912093960683998208e-1_f64) * t25372 * t98807 * t98809;
    let t98814 = F::cast_from(0.51405703062096148812e-1_f64) * t25386 * t98807 * t98809;
    let t98815 = t27312 * t689;
    let t98817 = F::cast_from(0.51405703062096148812e-1_f64) * t92838 * t98815;
    (t98785, t98803, t98806, t98811, t98814, t98815, t98817)
}
