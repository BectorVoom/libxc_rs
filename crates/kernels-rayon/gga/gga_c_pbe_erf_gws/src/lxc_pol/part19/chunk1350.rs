//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1350/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1350(t12275: f64, t14825: f64, t3931: f64, t810: f64, t3703: f64, t944: f64, t52112: f64, t57779: f64, t15567: f64, t945: f64, t12263: f64, t13756: f64, t14364: f64, t14831: f64, t15101: f64, t15124: f64, t3928: f64, t3946: f64, t4062: f64, t4066: f64, t4120: f64, t54792: f64, t56018: f64, t56027: f64, t57785: f64, t9807: f64) -> (f64, f64, f64, f64) {
    let t57809 = t12275 * t14825;
    let t57820 = t3931 * t810;
    let t57830 = t3703 * t944;
    let t57860 = t52112 * t57779;
    let t57883 = t15567 * t945;
    let t57889 = -t12263 * t4062 * t4120 - 6.0_f64 * t13756 * t4120 * t57830 - 6.0_f64 * t14364 * t15124 * t3946 - t14364 * t3928 * t4062 - 6.0_f64 * t14825 * t15101 * t3946 + 4.0_f64 * t14831 * t4062 * t54792 + 3.0_f64 * t3946 * t4066 * t9807 - 6.0_f64 * t3946 * t4120 * t56018 - 6.0_f64 * t3946 * t4120 * t56027 - 3.0_f64 * t3946 * t4120 * t57785 + 3.0_f64 * t3946 * t57883 * t810 - 6.0_f64 * t57860;
    (t57809, t57820, t57883, t57889)
}
