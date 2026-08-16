//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 934/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk934(t2900: f64, t513: f64, t1576: f64, t981: f64, t1578: f64, t985: f64, t2919: f64, t520: f64, t1590: f64, t5753: f64, t5755: f64, t5776: f64, t5863: f64, t5864: f64, t5866: f64, t5874: f64, t8117: f64, t8137: f64, t8142: f64, t8145: f64, t8171: f64, t8174: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8206 = t2900 * t513;
    let t8209 = t981 * t1576;
    let t8218 = t985 * t1578;
    let t8221 = t2919 * t520;
    let t8224 = t985 * t1590;
    let t8230 = t5753 - t5755 - t5863 + t8117 - t5776 - t8137 + t8142 + t8145 + t8171 + t8174 - 0.15326711111111111111e1_f64 * t5864 - 0.1724255e1_f64 * t5866 + 0.57475166666666666666e0_f64 * t5874;
    (t8206, t8209, t8218, t8221, t8224, t8230)
}
