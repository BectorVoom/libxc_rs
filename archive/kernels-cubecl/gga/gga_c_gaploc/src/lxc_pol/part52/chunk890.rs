//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 890/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk890<F: Float>(t11016: F, t11798: F, t36477: F, t955: F, t3470: F, t37057: F, t36512: F, t41339: F, t10742: F, t10867: F, t900: F, t44130: F) -> (F, F, F, F, F, F) {
    let t45933 = F::cast_from(0.7150097990370085334e0_f64) * t11798 * t11016;
    let t45939 = F::cast_from(0.23833659967900284446e0_f64) * t955 * t36477;
    let t45946 = F::cast_from(0.10725146985555128001e1_f64) * t37057 * t3470;
    let t45947 = t36512 * t41339;
    let t45950 = t10867 * t900 * t10742;
    let t45953 = F::cast_from(0.17875244975925213335e0_f64) * t44130;
    (t45933, t45939, t45946, t45947, t45950, t45953)
}
