//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1098/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1098(t76017: f64, t15001: f64, t551: f64, t70230: f64, t71744: f64, t73480: f64, t739: f64, t75993: f64, t75997: f64, t76000: f64, t76002: f64, t76021: f64, t76025: f64, t78423: f64, t78427: f64, t78431: f64, t78434: f64, t78436: f64, t78438: f64) -> (f64, f64) {
    let t80395 = 0.29085809927086856922e-4_f64 * t76017;
    let t80398 = t15001 * t551;
    let t80401 = -0.58171619854173713844e-5_f64 * t75993 + 0.58171619854173713844e-5_f64 * t75997 - 0.31062809106223861414e-2_f64 * t76000 + t76002 + t78423 - t70230 - t80395 + t71744 + t78427 + 0.76860658247009135562e-5_f64 * t76021 - t78431 - t78434 - 0.40878380883436523435e-5_f64 * t76025 - 0.59871208509319042821e-1_f64 * t739 * t80398 + t78436 + t73480 + t78438;
    (t80398, t80401)
}
