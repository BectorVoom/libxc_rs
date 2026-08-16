//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1208/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1208(t13846: f64, t4414: f64, t13826: f64, t840: f64, t13837: f64, t13822: f64, t8801: f64, t13972: f64, t14118: f64, t13772: f64, t2367: f64, t4002: f64, t4474: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51745 = t4414 * t13846;
    let t51756 = t840 * t13826;
    let t51758 = t4414 * t13837;
    let t51769 = t8801 * t13822;
    let t51771 = t13972 * t14118;
    let t51781 = t2367 * t13772;
    let t51788 = t4474 * t4002;
    (t51745, t51756, t51758, t51769, t51771, t51781, t51788)
}
