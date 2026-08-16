//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 802/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk802(t12916: f64, t12961: f64, t3675: f64, t985: f64, t3683: f64, t3637: f64, t967: f64, t10168: f64, t10170: f64, t12906: f64, t12907: f64, t12913: f64, t12931: f64, t12934: f64, t12946: f64, t12947: f64, t12951: f64, t12960: f64, t133: f64, t2911: f64, t2912: f64, t5753: f64, t5755: f64, t5776: f64, t5863: f64, t8252: f64) -> (f64, f64, f64, f64, f64) {
    let t12962 = t12916 + t12961;
    let t12970 = t3675 * t985;
    let t12973 = t985 * t3683;
    let t12978 = t967 * t3637;
    let t12987 = -t5863 + t12934 - t12906 + t12907 - 0.2069106e2_f64 * t133 * t12913 + 0.15518295e2_f64 * t2911 * t2912 * t12978 + t5753 - t5755 - t12947 - t12951 - 0.51727649999999999999e1_f64 * t10168 + 0.1724255e1_f64 * t10170 - t5776 - t12960 + t12946 - 0.1724255e1_f64 * t133 * t12931 - 0.22990066666666666666e1_f64 * t8252;
    (t12962, t12970, t12973, t12978, t12987)
}
