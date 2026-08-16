//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1031/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1031(t12978: f64, t2911: f64, t8236: f64, t133: f64, t42680: f64, t42661: f64, t42304: f64, t525: f64, t13062: f64, t751: f64, t12381: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42806 = t2911 * t8236 * t12978;
    let t42825 = t133 * t42680;
    let t42827 = t133 * t42661;
    let t42842 = t525 * t42304;
    let t42848 = t751 * t13062;
    let t42876 = t532 * t12381;
    (t42806, t42825, t42827, t42842, t42848, t42876)
}
