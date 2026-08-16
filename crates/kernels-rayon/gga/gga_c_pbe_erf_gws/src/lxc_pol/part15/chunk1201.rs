//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1201/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1201(t816: f64, t837: f64, t2080: f64, t2084: f64, t833: f64, t13800: f64, t13972: f64, t13893: f64, t3997: f64, t2238: f64, t4386: f64, t13842: f64, t9270: f64) -> (f64, f64, f64, f64, f64) {
    let t51502 = t816 * t837;
    let t51505 = t2080 * t2084 * t51502 * t833;
    let t51507 = t13972 * t13800;
    let t51509 = t13893 * t3997;
    let t51511 = t4386 * t2238;
    let t51524 = t9270 * t13842;
    (t51505, t51507, t51509, t51511, t51524)
}
