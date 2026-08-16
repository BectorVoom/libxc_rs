//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1073/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1073(t1356: f64, t78022: f64, t77980: f64, t2392: f64, t739: f64, t8264: f64, t2211: f64, t8924: f64, t76027: f64, t70212: f64, t70229: f64, t71744: f64, t71755: f64, t75993: f64, t75997: f64, t76000: f64, t76002: f64, t76017: f64, t76021: f64, t76025: f64, t78087: f64) -> f64 {
    let t78423 = 0.39914139006212695214e-1_f64 * t1356 * t78022;
    let t78427 = 0.39914139006212695214e-1_f64 * t1356 * t77980;
    let t78430 = t739 * t8264 * t2392;
    let t78431 = 0.2993560425465952141e-1_f64 * t78430;
    let t78433 = t739 * t2211 * t8924;
    let t78434 = 0.2993560425465952141e-1_f64 * t78433;
    let t78436 = 0.38430329123504567781e-4_f64 * t76027;
    let t78437 = t70212 - 0.58171619854173713846e-5_f64 * t75993 + 0.58171619854173713846e-5_f64 * t75997 - 0.31062809106223861415e-2_f64 * t76000 - 0.59871208509319042821e-1_f64 * t739 * t78087 + t76002 + t78423 - 0.12263514265030957031e-4_f64 * t70229 - 0.29085809927086856923e-4_f64 * t76017 + t71744 + t78427 + 0.76860658247009135557e-5_f64 * t76021 - t78431 - t78434 - 0.40878380883436523436e-5_f64 * t76025 + t78436 + t71755;
    t78437
}
