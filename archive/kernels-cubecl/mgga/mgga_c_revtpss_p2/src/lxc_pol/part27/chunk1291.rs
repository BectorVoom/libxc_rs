//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1291/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1291<F: Float>(t25178: F, t7235: F, t10416: F, t7003: F, t1937: F, t49693: F, t13435: F, t6993: F, t49856: F, t18163: F, t25188: F, t7239: F) -> (F, F, F, F, F, F, F) {
    let t95058 = F::cast_from(6.0_f64) * t7235 * t25178;
    let t95066 = F::cast_from(6.0_f64) * t10416 * t7003;
    let t95068 = F::cast_from(6.0_f64) * t49693 * t1937;
    let t95070 = F::cast_from(12.0_f64) * t13435 * t6993;
    let t95073 = F::cast_from(2.0_f64) * t49856 * t1937;
    let t95075 = F::cast_from(6.0_f64) * t18163 * t6993;
    let t95081 = F::cast_from(9.0_f64) * t25188 * t7239;
    (t95058, t95066, t95068, t95070, t95073, t95075, t95081)
}
