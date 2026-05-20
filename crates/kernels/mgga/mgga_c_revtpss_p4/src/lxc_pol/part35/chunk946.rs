//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 946/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk946<F: Float>(t11479: F, t11480: F, t18919: F, t18924: F, t18934: F, t19002: F, t19004: F, t19009: F, t23521: F, t23523: F, t23536: F, t23538: F, t23541: F, t23543: F) -> F {
    let t23693 = F::cast_from(0.20128333333333333333e0_f64) * t18919 - F::cast_from(0.60385000000000000001e0_f64) * t18924 + F::cast_from(0.30192500000000000001e0_f64) * t18934 - t11479 - t11480 + F::new(0.5519e-1) * t19002 - F::new(0.33114e0) * t19004 + F::new(0.16557e0) * t19009 - F::new(0.3883875e1) * t23521 + F::cast_from(0.247573125e0_f64) * t23523 + F::new(0.258925e1) * t23536 + F::new(0.16504875e0) * t23538 + F::new(0.19419375e1) * t23541 - F::cast_from(0.412621875e-1_f64) * t23543;
    t23693
}
