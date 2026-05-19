//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 936/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk936<F: Float>(t30194: F, t30197: F, t30199: F, t30229: F, t30232: F, t30238: F, t30242: F, t30246: F, t30339: F, t30396: F, t30405: F, t30421: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t32385 = F::cast_from(0.31448092289604152069e-3_f64) * t30194;
    let t32386 = F::cast_from(0.62896184579208304137e-3_f64) * t30197;
    let t32387 = F::cast_from(0.25724410870841842183e-2_f64) * t30199;
    let t32397 = F::cast_from(0.56606566121287473723e-2_f64) * t30229;
    let t32398 = F::cast_from(0.83861579438944405516e-2_f64) * t30232;
    let t32401 = F::cast_from(0.21437009059034868486e-3_f64) * t30238;
    let t32403 = F::cast_from(0.42874018118069736972e-3_f64) * t30242;
    let t32404 = F::cast_from(0.68026775414003982662e-1_f64) * t30246;
    let t32435 = F::cast_from(0.12862205435420921092e-2_f64) * t30339;
    let t32456 = F::new(5.0) / F::new(64.0) * t30396;
    let t32458 = F::cast_from(0.25724410870841842183e-2_f64) * t30405;
    let t32461 = F::cast_from(0.37737710747524982482e-2_f64) * t30421;
    (t32385, t32386, t32387, t32397, t32398, t32401, t32403, t32404, t32435, t32456, t32458, t32461)
}
