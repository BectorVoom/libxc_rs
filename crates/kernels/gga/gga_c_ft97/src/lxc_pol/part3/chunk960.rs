//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 960/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk960<F: Float>(t18096: F, t18099: F, t18102: F, t18105: F, t18107: F, t18110: F, t18113: F, t18115: F, t18118: F, t18121: F, t18125: F, t10883: F, t13538: F, t13544: F, t14544: F, t14553: F, t18826: F, t18831: F, t18834: F, t3750: F, t4068: F, t688: F, t9558: F) -> F {
    let t18851 = F::cast_from(0.3209574074074074074e-1_f64) * t18096 - F::cast_from(0.1604787037037037037e0_f64) * t18099 + F::cast_from(0.57772333333333333332e0_f64) * t18102 + F::cast_from(0.38514888888888888888e0_f64) * t18105 - F::cast_from(0.9628722222222222222e-1_f64) * t18107 - F::cast_from(0.86658499999999999998e0_f64) * t18110 - F::cast_from(0.11554466666666666666e1_f64) * t18113 + F::cast_from(0.4814361111111111111e-1_f64) * t18115 - F::cast_from(0.9628722222222222222e-1_f64) * t18118 + F::cast_from(0.28886166666666666666e0_f64) * t18121 - F::cast_from(0.14443083333333333333e0_f64) * t18125;
    let t18852 = F::new(0.1760655e0) * t18826 * t688 - F::new(0.234754e0) * t4068 * t3750 - F::new(0.117377e0) * t18831 * t688 + F::new(0.234754e0) * t18834 - t10883 - F::cast_from(0.6419148148148148148e-1_f64) * t9558 - F::cast_from(0.12838296296296296296e0_f64) * t13538 + t14544 - t14553 - F::cast_from(0.19257444444444444444e0_f64) * t13544 + t18851;
    t18852
}
