//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 717/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk717(t4881: f64, t4885: f64, t4890: f64, t4895: f64, t4900: f64, t4905: f64, t4907: f64, t4910: f64, t4912: f64, t4915: f64, t4917: f64, t4922: f64, t4926: f64, t4932: f64, t4937: f64, t4984: f64) -> f64 {
    let t5899 = -t4881 + t4885 + t4890 - t4895 - t4900 + t4905 + t4907 + t4910 + t4912 + t4915 - t4917 - t4922 + t4926 + t4932 + t4937 + t4984;
    t5899
}
