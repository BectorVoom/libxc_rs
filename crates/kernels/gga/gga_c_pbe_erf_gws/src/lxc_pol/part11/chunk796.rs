//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 796/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk796<F: Float>(t12526: F, t12579: F, t12609: F, t12663: F, t12742: F, t12772: F, t12808: F, t12879: F, t12323: F, t41: F, t11268: F, t163: F, t164: F, t169: F, t171: F, t5999: F, t6003: F, t6005: F, t6012: F, t6015: F, t8471: F, t8474: F, t8478: F, t8490: F) -> (F, F, F) {
    let t12882 = t12526 + t12579 + t12609 + t12663 + t12742 + t12772 + t12808 + t12879;
    let t12891 = t41 * t12323;
    let t12895 = -F::new(0.53884053046145740922e-2) * t169 * t171 * t12882 * t163 - F::new(0.94516221669423353502e-1) * t11268 - F::new(0.18903244333884670701e0) * t8474 + t5999 + F::new(0.18903244333884670701e0) * t8478 - F::new(0.94516221669423353502e-1) * t8490 + t6003 - t6005 + t6012 + t6015 - F::new(0.31505407223141117834e-1) * t12891 * t164 - F::new(0.71845404061527654564e-1) * t8471;
    (t12882, t12891, t12895)
}
