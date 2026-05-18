//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 994/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk994<F: Float>(t35190: F, t1181: F, t20311: F, t7351: F, t7426: F, t1165: F, t21118: F, t8600: F, t7637: F, t8555: F, t1967: F, t8549: F) -> (F, F, F, F, F) {
    let t35191 = F::new(0.47172138434406228102e-2) * t35190;
    let t35194 = t7426 * t1181 * t7351 * t20311;
    let t35195 = F::new(0.18868855373762491241e-2) * t35194;
    let t35198 = t7426 * t1165 * t8600 * t21118;
    let t35199 = F::new(0.37737710747524982482e-2) * t35198;
    let t35204 = t7637 * t8555;
    let t35210 = t1967 * t8549;
    (t35191, t35195, t35199, t35204, t35210)
}
