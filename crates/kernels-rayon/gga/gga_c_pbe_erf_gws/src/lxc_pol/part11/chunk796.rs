//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 796/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk796(t12526: f64, t12579: f64, t12609: f64, t12663: f64, t12742: f64, t12772: f64, t12808: f64, t12879: f64, t12323: f64, t41: f64, t11268: f64, t163: f64, t164: f64, t169: f64, t171: f64, t5999: f64, t6003: f64, t6005: f64, t6012: f64, t6015: f64, t8471: f64, t8474: f64, t8478: f64, t8490: f64) -> (f64, f64, f64) {
    let t12882 = t12526 + t12579 + t12609 + t12663 + t12742 + t12772 + t12808 + t12879;
    let t12891 = t41 * t12323;
    let t12895 = -0.53884053046145740922e-2_f64 * t169 * t171 * t12882 * t163 - 0.94516221669423353502e-1_f64 * t11268 - 0.18903244333884670701e0_f64 * t8474 + t5999 + 0.18903244333884670701e0_f64 * t8478 - 0.94516221669423353502e-1_f64 * t8490 + t6003 - t6005 + t6012 + t6015 - 0.31505407223141117834e-1_f64 * t12891 * t164 - 0.71845404061527654564e-1_f64 * t8471;
    (t12882, t12891, t12895)
}
