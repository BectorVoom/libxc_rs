//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 975/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk975<F: Float>(t13358: F, t13446: F, t10289: F, t10292: F, t1317: F, t13296: F, t13298: F, t13309: F, t13312: F, t13317: F, t1976: F, t1981: F, t3418: F, t3423: F, t3486: F, t4570: F, t4626: F, t578: F, t619: F, t7682: F, t7690: F, t91: F) -> (F, F) {
    let t13447 = t13358 + t13446;
    let t13450 = -F::cast_from(8.0_f64) * t10289 * t1317 + F::cast_from(40.0_f64) * t10292 * t3423 + t13296 * t91 - F::cast_from(4.0_f64) * t13298 * t619 - F::cast_from(120.0_f64) * t13309 * t7690 + F::cast_from(40.0_f64) * t13312 * t1981 + F::cast_from(20.0_f64) * t13317 * t1981 - F::cast_from(4.0_f64) * t13447 * t578 - F::cast_from(4.0_f64) * t1976 * t4626 - F::cast_from(8.0_f64) * t3418 * t3486 + F::cast_from(20.0_f64) * t4570 * t7682;
    (t13447, t13450)
}
