//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 857/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk857<F: Float>(t30055: F, t19: F, t3220: F, t336: F, t151: F, t177: F, t3558: F, t587: F, t2008: F, t980: F, t3646: F, t588: F) -> (F, F, F, F, F) {
    let t30056 = F::new(0.15724046144802076034e-3) * t30055;
    let t30058 = t3220 * t19 * t336;
    let t30077 = t151 * t587 * t3558 * t177;
    let t30078 = F::new(0.7558530601555998074e-1) * t30077;
    let t30080 = t980 * t2008 * t177;
    let t30081 = F::new(0.60023625365297631762e-2) * t30080;
    let t30083 = t3646 * t588 * t177;
    (t30056, t30058, t30078, t30081, t30083)
}
