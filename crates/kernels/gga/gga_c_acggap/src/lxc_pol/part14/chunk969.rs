//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 969/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk969<F: Float>(t30811: F, t4273: F, t129: F, t507: F, t7585: F, t7587: F, t30546: F, t8477: F, t1967: F, t8561: F, t30543: F, t8515: F) -> (F, F, F, F, F, F) {
    let t34340 = t30811 * t4273;
    let t34341 = F::cast_from(0.68598428988911579156e-2_f64) * t34340;
    let t34345 = t129 * t507;
    let t34347 = t7585 * t34345 * t7587;
    let t34348 = F::cast_from(0.14291339372689912324e-3_f64) * t34347;
    let t34349 = t30546 * t8477;
    let t34351 = t1967 * t8561;
    let t34352 = F::cast_from(0.37737710747524982482e-2_f64) * t34351;
    let t34361 = t30543 * t8515;
    (t34341, t34345, t34348, t34349, t34352, t34361)
}
