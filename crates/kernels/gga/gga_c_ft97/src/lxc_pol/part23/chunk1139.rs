//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1139/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1139<F: Float>(t28188: F, t8392: F, t1449: F, t9570: F, t28181: F, t28350: F, t28305: F, t24412: F, t737: F, t6932: F, t8232: F, t28302: F, t28295: F, t28395: F, t28312: F, t762: F, t9707: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t109875 = 2.0 / 27.0 * t8392 * t28188;
    let t109890 = t1449 * t9570;
    let t109900 = 4.0 / 81.0 * t8392 * t28181;
    let t109902 = 2.0 / 27.0 * t8392 * t28350;
    let t109925 = 2.0 / 27.0 * t8392 * t28305;
    let t109926 = t737 * t24412;
    let t109936 = t8232 * t6932;
    let t109960 = 4.0 / 3.0 * t8392 * t28302;
    let t109962 = 4.0 / 9.0 * t8392 * t28295;
    let t109968 = 2.0 / 27.0 * t8392 * t28395;
    let t109989 = 2.0 / 27.0 * t8392 * t28312;
    let t110010 = t9707 * t762;
    (t109875, t109890, t109900, t109902, t109925, t109926, t109936, t109960, t109962, t109968, t109989, t110010)
}
