//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 631/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk631(t4924: f64, t587: f64, t1791: f64, t642: f64, t1793: f64, t626: f64, t422: f64, t639: f64, t4872: f64, t4873: f64, t4876: f64, t4881: f64, t4885: f64, t4890: f64, t4895: f64, t4900: f64, t4905: f64, t4907: f64, t4910: f64, t4912: f64, t4915: f64, t4917: f64, t4922: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4926 = 8.0_f64 / 15.0_f64 * t587 * t4924;
    let t4927 = t642 * t1791;
    let t4928 = t1793 * t626;
    let t4929 = t4928 * t422;
    let t4930 = t4927 * t4929;
    let t4932 = 8.0_f64 / 15.0_f64 * t639 * t4930;
    let t4933 = -t4872 + 0.9973633333333333333e-1_f64 * t4873 + t4876 - t4881 + t4885 + t4890 - t4895 - t4900 + t4905 + t4907 + t4910 + t4912 + t4915 - t4917 - t4922 + t4926 + t4932;
    (t4926, t4927, t4928, t4929, t4930, t4932, t4933)
}
