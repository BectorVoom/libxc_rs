//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1080/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1080<F: Float>(t28676: F, t70456: F, t4246: F, t848: F, t2766: F, t2770: F, t5374: F, t15128: F, t2681: F, t2843: F, t2842: F, t1240: F, t799: F, t10491: F, t1255: F, t19782: F, t870: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t70786 = t28676 * t70456;
    let t71524 = t848 * t4246;
    let t71528 = t2766 * t4246;
    let t71624 = t2770 * t5374;
    let t72163 = t848 * t15128;
    let t72190 = t2681 * t2843;
    let t72231 = t5374 * t2842;
    let t72391 = t848 * t5374;
    let t72397 = t799 * t1240;
    let t72443 = t10491 * t1255;
    let t72745 = t19782 * t870;
    (t70786, t71524, t71528, t71624, t72163, t72190, t72231, t72391, t72397, t72443, t72745)
}
