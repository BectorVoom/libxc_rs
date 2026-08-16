//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 910/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk910<F: Float>(t18063: F, t701: F, t3799: F, t3810: F, t3807: F, t13616: F, t17780: F, t17727: F, t2320: F, t17732: F, t3806: F, t172: F, t228: F, t231: F, t4995: F) -> (F, F, F, F, F, F, F) {
    let t18064 = t701 * t18063;
    let t18066 = t3799 * t3810;
    let t18068 = t3799 * t3807;
    let t18070 = t13616 * t17780;
    let t18071 = t701 * t18070;
    let t18073 = t2320 * t17727;
    let t18074 = t701 * t18073;
    let t18076 = t3806 * t17732;
    let t18077 = t701 * t18076;
    let t18081 = t228 * t4995 * t172 * t231;
    (t18064, t18066, t18068, t18071, t18074, t18077, t18081)
}
