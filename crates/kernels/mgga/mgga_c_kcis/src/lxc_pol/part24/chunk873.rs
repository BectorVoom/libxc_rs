//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 873/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk873<F: Float>(t13710: F, t13712: F, t13717: F, t13842: F, t18645: F, t18650: F, t18655: F, t18659: F, t18661: F, t18664: F, t18667: F, t18669: F, t18674: F, t18679: F, t18683: F, t9691: F, t9790: F) -> F {
    let t19040 = -t9790 - F::new(0.79148148148148148147e-2) * t9691 - F::new(0.15829629629629629629e-1) * t13710 + F::new(0.79148148148148148147e-2) * t13712 - t13842 + F::new(0.23744444444444444444e-1) * t13717 + F::new(0.39574074074074074073e-2) * t18645 - F::new(0.19787037037037037037e-1) * t18650 + F::new(0.71233333333333333332e-1) * t18655 - F::new(0.47488888888888888888e-1) * t18659 - F::new(0.11872222222222222222e-1) * t18661 - F::new(0.10685e0) * t18664 + F::new(0.14246666666666666666e0) * t18667 + F::new(0.5936111111111111111e-2) * t18669 - F::new(0.11872222222222222222e-1) * t18674 + F::new(0.35616666666666666666e-1) * t18679 - F::new(0.17808333333333333333e-1) * t18683;
    t19040
}
