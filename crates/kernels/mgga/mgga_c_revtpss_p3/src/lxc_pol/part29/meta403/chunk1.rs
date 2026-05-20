//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1450/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1450<F: Float>(t15220: F, t923: F, t916: F, t11134: F, t11136: F, t11138: F, t11140: F, t11339: F, t11366: F, t11368: F, t11479: F, t11480: F) -> (F, F, F) {
    let t15221 = t923 * t15220;
    let t15230 = t916 * t15220;
    let t15232 = -t11479 - t11480 + F::new(0.16504875e0) * t15221 + F::cast_from(0.18396666666666666667e-1_f64) * t11339 - F::cast_from(0.20128333333333333334e0_f64) * t11138 - F::cast_from(0.26837777777777777778e0_f64) * t11134 + F::cast_from(0.10064166666666666667e0_f64) * t11140 + F::cast_from(0.67094444444444444447e-1_f64) * t11136 - F::cast_from(0.18396666666666666667e0_f64) * t11366 + F::new(0.5519e-1) * t11368 + F::new(0.258925e1) * t15230;
    (t15221, t15230, t15232)
}
