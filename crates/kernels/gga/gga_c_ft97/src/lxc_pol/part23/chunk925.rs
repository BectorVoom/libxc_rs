//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 925/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk925<F: Float>(t28386: F, t3886: F, t14182: F, t265: F, t27742: F, t729: F, t24789: F, t3898: F, t24793: F, t3881: F, t3887: F, t1443: F, t2486: F, t3893: F, t6918: F, t8392: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28387 = t28386 * t3886;
    let t28388 = t14182 * t28387;
    let t28392 = t729 * t265 * t27742;
    let t28395 = t24789 * t3898;
    let t28398 = t24793 * t3881;
    let t28401 = t24793 * t3887;
    let t28404 = t2486 * t1443;
    let t28405 = t28404 * t3893;
    let t28408 = t8392 * t6918;
    (t28387, t28388, t28392, t28395, t28398, t28401, t28404, t28405, t28408)
}
