//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 858/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk858<F: Float>(t30083: F, t177: F, t377: F, t7370: F, t2067: F, t3077: F, t7348: F, t1160: F, t7432: F, t7365: F, t4180: F, t3427: F, t7647: F) -> (F, F, F, F, F, F, F, F) {
    let t30084 = F::new(0.42874018118069736972e-3) * t30083;
    let t30088 = t377 * t7370 * t177;
    let t30089 = F::new(0.34013387707001991332e-1) * t30088;
    let t30090 = t3077 * t2067;
    let t30091 = t30090 * t7348;
    let t30105 = t1160 * t7432;
    let t30106 = t30105 * t7365;
    let t30120 = t4180 * t2067;
    let t30123 = t7647 * t3427;
    (t30084, t30089, t30090, t30091, t30105, t30106, t30120, t30123)
}
