//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 989/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk989(t3074: f64, t8847: f64, t814: f64, t857: f64, t858: f64, t856: f64, t6229: f64, t2170: f64, t2171: f64, t8840: f64, t2168: f64, t8821: f64, t8823: f64, t8826: f64, t8831: f64, t8832: f64, t8835: f64, t8839: f64, t8844: f64, t8846: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8848 = t3074 * t8847;
    let t8850 = t857 * t858 * t814;
    let t8851 = t856 * t8850;
    let t8853 = t8848 * t8851 / 32.0_f64;
    let t8854 = 35.0_f64 / 216.0_f64 * t6229;
    let t8856 = t2170 * t8840 * t2171;
    let t8858 = t2168 * t8856 / 24.0_f64;
    let t8859 = -t8821 + t8823 + t8826 + t8831 + t8832 + t8835 - t8839 + t8844 - t8846 - t8853 - t8854 + t8858;
    (t8848, t8853, t8854, t8856, t8858, t8859)
}
