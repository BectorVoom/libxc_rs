//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2337/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2337(t113: f64, t12545: f64, t12835: f64, t1393: f64, t24932: f64, t27903: f64, t4077: f64, t7266: f64, t91602: f64, t91606: f64, t91608: f64, t91610: f64, t91612: f64, t91623: f64, t91625: f64, t91627: f64, t91630: f64, t91637: f64, t91640: f64, t91642: f64, t91657: f64, t91662: f64, t94293: f64, t95965: f64) -> f64 {
    let t95970 = -t91602 - t91606 - t91608 - t91610 - t91612 + t91623 - t91625 - t91627 - t91630 - 2.0_f64 * t7266 * t12835 - 4.0_f64 * t24932 * t4077 - 4.0_f64 * t7266 * t12545 - t113 * (t94293 + t95965) + t91637 + 2.0_f64 * t27903 * t1393 + t91640 + t91642 - t91657 + t91662;
    t95970
}
