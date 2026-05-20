//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1915/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1915<F: Float>(t25365: F, t28360: F, t26485: F, t99466: F, t28377: F, t689: F, t25431: F, t25411: F, t102928: F, t25387: F, t28404: F, t28384: F) -> (F, F, F, F, F, F, F, F) {
    let t102945 = F::cast_from(0.25702851531048074406e-1_f64) * t25365 * t28360;
    let t102947 = F::cast_from(0.28912093960683998208e-1_f64) * t99466 * t26485;
    let t102951 = t28377 * t689;
    let t102953 = F::cast_from(0.14456046980341999104e-1_f64) * t25431 * t102951;
    let t102956 = F::cast_from(0.25702851531048074406e-1_f64) * t25411 * t102951;
    let t102964 = F::cast_from(0.51405703062096148812e-1_f64) * t25387 * t102928;
    let t102967 = t28404 * t689;
    let t102969 = F::cast_from(0.14456046980341999104e-1_f64) * t25431 * t102967;
    let t102971 = F::cast_from(0.25702851531048074406e-1_f64) * t25411 * t102967;
    let t102972 = t28384 * t689;
    (t102945, t102947, t102953, t102956, t102964, t102969, t102971, t102972)
}
