//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3519/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3519<F: Float>(t15700: F, t19992: F, t53405: F, t16226: F, t19997: F, t11710: F, t19777: F, t3091: F, t19644: F, t15596: F, t15605: F, t15611: F, t15618: F, t15688: F, t1664: F, t19707: F, t19722: F, t42967: F, t43043: F, t4912: F, t53800: F, t53855: F, t54289: F, t54341: F, t54348: F, t54542: F, t6268: F) -> F {
    let t66644 = t15700 * t53405 * t19992;
    let t66647 = t16226 * t53405 * t19997;
    let t66655 = t3091 * t11710 * t19777;
    let t66660 = t3091 * t11710 * t19644;
    let t66662 = -F::cast_from(0.17149607247227894789e-2_f64) * t43043 * t15688 * t1664 * t15605 - F::cast_from(0.60976381323476959249e-2_f64) * t54289 * t19707 + F::cast_from(0.42874018118069736972e-3_f64) * t54542 * t19722 - F::cast_from(0.85748036236139473944e-3_f64) * t53855 * t4912 - F::cast_from(0.3811023832717309953e-3_f64) * t54341 - F::cast_from(0.76220476654346199061e-3_f64) * t66644 + F::cast_from(0.76220476654346199061e-3_f64) * t66647 + F::cast_from(0.76220476654346199061e-3_f64) * t54348 - F::cast_from(0.30488190661738479624e-2_f64) * t42967 * t6268 - F::cast_from(0.17149607247227894789e-2_f64) * t53800 * t15611 + F::cast_from(0.3811023832717309953e-3_f64) * t66655 + F::cast_from(0.47637797908966374413e-3_f64) * t15618 * t15596 + F::cast_from(0.19055119163586549765e-3_f64) * t66660;
    t66662
}
