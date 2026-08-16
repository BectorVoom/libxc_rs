//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1996/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1996<F: Float>(t95822: F, t98815: F, t95537: F, t25310: F, t28360: F, t25365: F, t26485: F, t99466: F, t28377: F, t689: F, t25431: F, t102930: F, t102934: F, t102937: F, t14979: F, t7403: F, t95538: F, t95542: F, t95543: F, t95548: F) -> (F, F) {
    let t102939 = F::cast_from(0.28912093960683998208e-1_f64) * t95822 * t98815;
    let t102941 = F::cast_from(0.51405703062096148812e-1_f64) * t95537 * t98815;
    let t102943 = F::cast_from(0.14456046980341999104e-1_f64) * t25310 * t28360;
    let t102945 = F::cast_from(0.25702851531048074406e-1_f64) * t25365 * t28360;
    let t102947 = F::cast_from(0.28912093960683998208e-1_f64) * t99466 * t26485;
    let t102951 = t28377 * t689;
    let t102953 = F::cast_from(0.14456046980341999104e-1_f64) * t25431 * t102951;
    let t102954 = -F::cast_from(0.51405703062096148812e-1_f64) * t95538 - t102930 + t102934 - t102937 + t102939 - t102941 + t102943 - t102945 - t102947 - t95542 - F::cast_from(0.12851425765524037203e-1_f64) * t95543 - F::cast_from(0.65854491829355115987e0_f64) * t7403 * t14979 - t95548 - t102953;
    (t102951, t102954)
}
