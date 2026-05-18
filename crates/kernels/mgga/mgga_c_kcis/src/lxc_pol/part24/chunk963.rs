//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 963/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk963<F: Float>(t10945: F, t13710: F, t13712: F, t13717: F, t15432: F, t18645: F, t18650: F, t18655: F, t18659: F, t18661: F, t18664: F, t18667: F, t18669: F, t18674: F, t18679: F, t18683: F, t9691: F) -> F {
    let t20377 = -t10945 - F::new(0.76103703703703703703e-2) * t9691 - F::new(0.1522074074074074074e-1) * t13710 + F::new(0.761037037037037037e-2) * t13712 - t15432 + F::new(0.2283111111111111111e-1) * t13717 + F::new(0.3805185185185185185e-2) * t18645 - F::new(0.19025925925925925925e-1) * t18650 + F::new(0.68493333333333333331e-1) * t18655 - F::new(0.4566222222222222222e-1) * t18659 - F::new(0.11415555555555555555e-1) * t18661 - F::new(0.10274e0) * t18664 + F::new(0.13698666666666666666e0) * t18667 + F::new(0.57077777777777777777e-2) * t18669 - F::new(0.11415555555555555555e-1) * t18674 + F::new(0.34246666666666666666e-1) * t18679 - F::new(0.17123333333333333333e-1) * t18683;
    t20377
}
