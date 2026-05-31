//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1210/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1210<F: Float>(t19226: F, t954: F, t11134: F, t11574: F, t15127: F, t15189: F, t15363: F, t15364: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18944: F, t18948: F) -> (F, F) {
    let t19227 = t19226 * t954;
    let t19247 = -t11574 - F::cast_from(0.76103703703703703703e-2_f64) * t11134 - F::cast_from(0.1522074074074074074e-1_f64) * t15189 + F::cast_from(0.761037037037037037e-2_f64) * t15127 - t15363 + t15364 + F::cast_from(0.3805185185185185185e-2_f64) * t18919 - F::cast_from(0.19025925925925925925e-1_f64) * t18906 + F::cast_from(0.68493333333333333331e-1_f64) * t18911 - F::cast_from(0.2283111111111111111e-1_f64) * t18915 - F::cast_from(0.11415555555555555555e-1_f64) * t18924 - F::cast_from(0.10274e0_f64) * t18928 + F::cast_from(0.68493333333333333332e-1_f64) * t18932 + F::cast_from(0.57077777777777777777e-2_f64) * t18934 - F::cast_from(0.11415555555555555555e-1_f64) * t18939 + F::cast_from(0.34246666666666666666e-1_f64) * t18944 - F::cast_from(0.17123333333333333333e-1_f64) * t18948;
    (t19227, t19247)
}
