//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 908/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk908<F: Float>(t173: F, t5037: F, t701: F, t5041: F, t3799: F, t3803: F, t227: F, t4995: F, t9: F, t706: F, t3814: F, t13596: F, t13601: F, t13629: F, t13636: F, t13648: F, t18032: F, t9639: F) -> (F, F, F, F, F, F) {
    let t18034 = t173 * t5037;
    let t18035 = t701 * t18034;
    let t18037 = t173 * t5041;
    let t18038 = t701 * t18037;
    let t18040 = t3799 * t3803;
    let t18043 = t9 * t227 * t4995;
    let t18044 = t18043 * t706;
    let t18046 = t3799 * t3814;
    let t18048 = -F::new(0.85124811172839506172e-2) * t13596 + t13601 - F::new(0.14187468528806584362e-2) * t9639 - F::new(0.85124811172839506172e-2) * t13629 - t13636 - F::new(0.28374937057613168724e-2) * t13648 + F::new(0.21281202793209876543e-2) * t18032 + F::new(0.28374937057613168724e-2) * t18035 - F::new(0.42562405586419753087e-2) * t18038 - F::new(0.1134997482304526749e-1) * t18040 + F::new(0.62424861526748971193e-1) * t18044 + F::new(0.6809984893827160494e-1) * t18046;
    (t18035, t18038, t18040, t18044, t18046, t18048)
}
