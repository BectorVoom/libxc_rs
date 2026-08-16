//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1260/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1260(t14766: f64, t833: f64, t8945: f64, t1114: f64, t51922: f64, t14138: f64, t14733: f64, t51042: f64, t14001: f64, t3214: f64, t3950: f64, t850: f64, t9170: f64) -> (f64, f64, f64, f64, f64) {
    let t53889 = t8945 * t14766 * t833;
    let t53891 = t1114 * t51922;
    let t53892 = t53891 * t14138;
    let t53894 = t14733 * t51042;
    let t53896 = t14001 * t3214;
    let t53904 = t850 * t9170 * t3950 * t833;
    (t53889, t53892, t53894, t53896, t53904)
}
