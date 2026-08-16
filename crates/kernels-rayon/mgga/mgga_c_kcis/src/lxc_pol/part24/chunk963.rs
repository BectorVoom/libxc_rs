//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 963/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk963(t10945: f64, t13710: f64, t13712: f64, t13717: f64, t15432: f64, t18645: f64, t18650: f64, t18655: f64, t18659: f64, t18661: f64, t18664: f64, t18667: f64, t18669: f64, t18674: f64, t18679: f64, t18683: f64, t9691: f64) -> f64 {
    let t20377 = -t10945 - 0.76103703703703703703e-2_f64 * t9691 - 0.1522074074074074074e-1_f64 * t13710 + 0.761037037037037037e-2_f64 * t13712 - t15432 + 0.2283111111111111111e-1_f64 * t13717 + 0.3805185185185185185e-2_f64 * t18645 - 0.19025925925925925925e-1_f64 * t18650 + 0.68493333333333333331e-1_f64 * t18655 - 0.4566222222222222222e-1_f64 * t18659 - 0.11415555555555555555e-1_f64 * t18661 - 0.10274e0_f64 * t18664 + 0.13698666666666666666e0_f64 * t18667 + 0.57077777777777777777e-2_f64 * t18669 - 0.11415555555555555555e-1_f64 * t18674 + 0.34246666666666666666e-1_f64 * t18679 - 0.17123333333333333333e-1_f64 * t18683;
    t20377
}
