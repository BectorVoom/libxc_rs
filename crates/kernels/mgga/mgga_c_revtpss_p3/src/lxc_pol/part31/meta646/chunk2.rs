//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2114/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2114<F: Float>(t18437: F, t7045: F, t18348: F, t1945: F, t807: F, t25266: F, t6019: F, t6024: F, t93054: F, t103297: F, t99020: F, t99022: F, t99024: F, t99027: F, t99030: F, t99034: F, t99042: F) -> F {
    let t106058 = t7045 * t18437;
    let t106061 = t807 * t1945 * t18348;
    let t106063 = t25266 * t6019;
    let t106065 = t93054 * t6024;
    let t106067 = F::cast_from(0.85748036236139473945e-2_f64) * t106058 + t99020 - t99022 - t99024 - t99027 + t99030 + t99034 - t103297 + t99042 + F::cast_from(0.57165357490759649296e-4_f64) * t106061 + F::cast_from(0.20007875121765877254e-2_f64) * t106063 - F::cast_from(0.40015750243531754507e-2_f64) * t106065;
    t106067
}
