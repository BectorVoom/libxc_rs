//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 785/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk785<F: Float>(t151: F, t177: F, t3558: F, t587: F, t2008: F, t980: F, t3646: F, t588: F, t377: F, t7370: F, t2067: F, t3077: F, t7348: F, t1160: F, t7432: F, t7365: F) -> (F, F, F, F, F, F, F, F) {
    let t30077 = t151 * t587 * t3558 * t177;
    let t30078 = 0.7558530601555998074e-1 * t30077;
    let t30080 = t980 * t2008 * t177;
    let t30081 = 0.60023625365297631762e-2 * t30080;
    let t30083 = t3646 * t588 * t177;
    let t30084 = 0.42874018118069736972e-3 * t30083;
    let t30088 = t377 * t7370 * t177;
    let t30089 = 0.34013387707001991332e-1 * t30088;
    let t30090 = t3077 * t2067;
    let t30091 = t30090 * t7348;
    let t30105 = t1160 * t7432;
    let t30106 = t30105 * t7365;
    (t30078, t30081, t30084, t30089, t30090, t30091, t30105, t30106)
}
