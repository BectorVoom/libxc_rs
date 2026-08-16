//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1386/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1386(t55556: f64, t55557: f64, t57082: f64, t57086: f64, t57088: f64, t57090: f64, t57092: f64, t57094: f64, t57096: f64, t57098: f64, t57100: f64, t57102: f64, t57104: f64) -> f64 {
    let t58709 = -t57082 / 384.0_f64 + t57086 / 24.0_f64 - t57088 / 12.0_f64 - t57090 / 48.0_f64 - t57092 / 384.0_f64 - 5.0_f64 / 96.0_f64 * t57094 + t57096 / 48.0_f64 + t57098 / 24.0_f64 + t55556 + t57100 / 48.0_f64 - t57102 / 48.0_f64 - t55557 + 7.0_f64 / 576.0_f64 * t57104;
    t58709
}
