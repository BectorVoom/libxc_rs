//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 709/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk709<F: Float>(t18459: F, t3892: F, t9803: F, t13839: F, t3887: F, t1160: F, t2486: F, t3893: F, t4635: F, t713: F, t2600: F, t2599: F, t14159: F, t3876: F, t3881: F, t4969: F, t724: F, t773: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18460 = t3892 * t18459;
    let t18461 = t9803 * t18460;
    let t18464 = t13839 * t3887;
    let t18467 = t2486 * t1160;
    let t18468 = t18467 * t3893;
    let t18471 = t4635 * t713;
    let t18472 = t2600 * t18471;
    let t18473 = t2599 * t18472;
    let t18476 = t14159 * t3876;
    let t18479 = t13839 * t3881;
    let t18483 = t724 * t773 * t4969;
    (t18460, t18461, t18464, t18468, t18471, t18472, t18473, t18476, t18479, t18483)
}
