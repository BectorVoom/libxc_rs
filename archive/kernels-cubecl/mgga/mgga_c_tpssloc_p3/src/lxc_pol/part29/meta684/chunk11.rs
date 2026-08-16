//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2337/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2337<F: Float>(t113: F, t12545: F, t12835: F, t1393: F, t24932: F, t27903: F, t4077: F, t7266: F, t91602: F, t91606: F, t91608: F, t91610: F, t91612: F, t91623: F, t91625: F, t91627: F, t91630: F, t91637: F, t91640: F, t91642: F, t91657: F, t91662: F, t94293: F, t95965: F) -> F {
    let t95970 = -t91602 - t91606 - t91608 - t91610 - t91612 + t91623 - t91625 - t91627 - t91630 - F::cast_from(2.0_f64) * t7266 * t12835 - F::cast_from(4.0_f64) * t24932 * t4077 - F::cast_from(4.0_f64) * t7266 * t12545 - t113 * (t94293 + t95965) + t91637 + F::cast_from(2.0_f64) * t27903 * t1393 + t91640 + t91642 - t91657 + t91662;
    t95970
}
