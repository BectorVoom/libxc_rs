//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1182/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1182(t1: f64, t2049: f64, t2057: f64, t2062: f64, t18885: f64, t18899: f64, t18959: f64, t18961: f64, t18964: f64, t18968: f64, t18970: f64, t18973: f64, t18975: f64, t18977: f64, t18979: f64, t18981: f64, t18983: f64) -> (f64, f64) {
    let t20987 = t2049 * t2057 * t1 * t2062;
    let t20988 = 0.37963457796989083263e1_f64 * t20987;
    let t20989 = t18885 - t18959 - t18961 + t18964 - t18968 + t18970 - t18973 + t18975 - t18977 + t18979 + t18981 - t20988 - t18983 - t18899;
    (t20988, t20989)
}
