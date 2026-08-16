//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1044/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1044(t1550: f64, t7778: f64, t8377: f64, t1632: f64, t2064: f64, t3928: f64, t2373: f64, t7561: f64, t2283: f64, t7944: f64, t36424: f64, t36590: f64, t36594: f64, t36601: f64, t36710: f64, t41690: f64, t41694: f64, t41696: f64, t41701: f64, t41706: f64, t41713: f64, t41717: f64, t41719: f64, t4965: f64, t530: f64, t8804: f64) -> f64 {
    let t41722 = t1550 * t7778 * t8377;
    let t41723 = 0.15965655602485078085e0_f64 * t41722;
    let t41725 = t3928 * t2064 * t1632;
    let t41726 = 0.47896966807455234256e0_f64 * t41725;
    let t41727 = t2373 * t7561;
    let t41730 = t7944 * t2283;
    let t41732 = 0.79828278012425390428e-1_f64 * t4965 * t8804 - 0.4726e1_f64 * t530 * t36710 + 0.25538759935978703639e-4_f64 * t41690 - 0.25538759935978703639e-4_f64 * t41694 + 0.1064114997332445985e-4_f64 * t41696 - 0.63846899839946759096e-4_f64 * t41701 - 0.25538759935978703638e-4_f64 * t41706 + 0.18183107769496894486e-1_f64 * t36590 + 0.90915538847484472429e-2_f64 * t36594 - 0.2363e1_f64 * t530 * t36424 + 0.8980681276397856423e0_f64 * t41713 + t41717 - 0.5987120850931904282e-1_f64 * t41719 - t41723 - t41726 + 0.33335697577410973224e-1_f64 * t41727 + 2.0_f64 * t36601 - 0.42564599893297839398e-5_f64 * t41730;
    t41732
}
