//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 938/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk938<F: Float>(t3470: F, t37061: F, t36590: F, t955: F, t11016: F, t11798: F, t36477: F, t1: F, t106: F, t13525: F, t316: F, t37057: F) -> (F, F, F, F, F, F) {
    let t45922 = F::cast_from(0.10725146985555128001e1_f64) * t37061 * t3470;
    let t45931 = F::cast_from(0.23833659967900284446e0_f64) * t955 * t36590;
    let t45933 = F::cast_from(0.7150097990370085334e0_f64) * t11798 * t11016;
    let t45939 = F::cast_from(0.23833659967900284446e0_f64) * t955 * t36477;
    let t45942 = t13525 * t1 * t106 * t316;
    let t45946 = F::cast_from(0.10725146985555128001e1_f64) * t37057 * t3470;
    (t45922, t45931, t45933, t45939, t45942, t45946)
}
