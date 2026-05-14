//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 765/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk765<F: Float>(t12905: F, t3680: F, t317: F, t3675: F, t305: F, t1190: F, t3640: F, t3679: F, t1175: F, t3587: F, t3598: F, t3651: F, t12831: F, t4271: F, t12: F) -> (F, F, F, F, F, F) {
    let t12907 = 0.48245472966453314466e2 * t12905 * t3680;
    let t12909 = 1.0 / t3675 / t317;
    let t12910 = t305 * t12909;
    let t12911 = t3640 * t1190;
    let t12912 = t12911 * t3679;
    let t12914 = 0.96490945932906628932e2 * t12910 * t12912;
    let t12916 = t3598 * t1175 * t3587;
    let t12919 = t3651 * t1175 * t3587;
    let t12921 = t4271 * t12831;
    let t12922 = t12 * t12921;
    (t12907, t12911, t12914, t12916, t12919, t12922)
}
