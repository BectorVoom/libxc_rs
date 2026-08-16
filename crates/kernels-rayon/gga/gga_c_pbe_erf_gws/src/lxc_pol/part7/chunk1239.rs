//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1239/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1239(t2423: f64, t18899: f64, t18961: f64, t18964: f64, t18968: f64, t18970: f64, t18973: f64, t18975: f64, t18977: f64, t18979: f64, t18981: f64, t18983: f64, t2053: f64, t2054: f64, t2074: f64, t2075: f64, t20988: f64, t2182: f64, t2429: f64, t321: f64, t6855: f64, t804: f64, t810: f64, t8524: f64) -> f64 {
    let t21890 = t2423 * t2423;
    let t21905 = -3.0_f64 * t2053 * t21890 * t321 - 18.0_f64 * t2054 * t2074 * t804 - 36.0_f64 * t2054 * t2182 * t2429 + 24.0_f64 * t6855 * t804 * t810 + 36.0_f64 * t2075 * t8524 - t18899 - t18961 + t18964 - t18968 + t18970 - t18973 + t18975 - t18977 + t18979 + t18981 - t18983 - t20988;
    t21905
}
