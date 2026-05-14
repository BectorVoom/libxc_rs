//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 791/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk791<F: Float>(t18043: F, t706: F, t3799: F, t3814: F, t13596: F, t13601: F, t13629: F, t13636: F, t13648: F, t18032: F, t18035: F, t18038: F, t18040: F, t9639: F, t16579: F, t704: F) -> (F, F, F, F) {
    let t18044 = t18043 * t706;
    let t18046 = t3799 * t3814;
    let t18048 = -0.85124811172839506172e-2 * t13596 + t13601 - 0.14187468528806584362e-2 * t9639 - 0.85124811172839506172e-2 * t13629 - t13636 - 0.28374937057613168724e-2 * t13648 + 0.21281202793209876543e-2 * t18032 + 0.28374937057613168724e-2 * t18035 - 0.42562405586419753087e-2 * t18038 - 0.1134997482304526749e-1 * t18040 + 0.62424861526748971193e-1 * t18044 + 0.6809984893827160494e-1 * t18046;
    let t18049 = t704 * t16579;
    (t18044, t18046, t18048, t18049)
}
