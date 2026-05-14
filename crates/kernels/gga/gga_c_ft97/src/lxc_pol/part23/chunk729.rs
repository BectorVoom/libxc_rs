//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 729/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk729<F: Float>(t2697: F, t4977: F, t18127: F, t801: F, t18096: F, t18099: F, t18102: F, t18105: F, t18107: F, t18110: F, t18113: F, t18115: F, t18118: F, t18121: F, t18125: F, t10883: F, t13538: F, t13544: F, t14544: F, t14553: F, t18826: F, t3750: F, t4068: F, t688: F, t9558: F) -> (F, F, F) {
    let t18831 = t2697 * t4977;
    let t18834 = t801 * t18127;
    let t18851 = 0.3209574074074074074e-1 * t18096 - 0.1604787037037037037e0 * t18099 + 0.57772333333333333332e0 * t18102 + 0.38514888888888888888e0 * t18105 - 0.9628722222222222222e-1 * t18107 - 0.86658499999999999998e0 * t18110 - 0.11554466666666666666e1 * t18113 + 0.4814361111111111111e-1 * t18115 - 0.9628722222222222222e-1 * t18118 + 0.28886166666666666666e0 * t18121 - 0.14443083333333333333e0 * t18125;
    let t18852 = 0.1760655e0 * t18826 * t688 - 0.234754e0 * t4068 * t3750 - 0.117377e0 * t18831 * t688 + 0.234754e0 * t18834 - t10883 - 0.6419148148148148148e-1 * t9558 - 0.12838296296296296296e0 * t13538 + t14544 - t14553 - 0.19257444444444444444e0 * t13544 + t18851;
    (t18831, t18834, t18852)
}
