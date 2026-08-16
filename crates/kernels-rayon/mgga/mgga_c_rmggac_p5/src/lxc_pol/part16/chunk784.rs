//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 784/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk784(t35716: f64, t35728: f64, t35776: f64, t35781: f64, t35786: f64, t35798: f64, t2265: f64, t4036: f64, t36330: f64, t1347: f64, t2244: f64, t36504: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37822 = 0.26021382394247697185e-3_f64 * t35716;
    let t37825 = 0.13010691197123848592e-3_f64 * t35728;
    let t37848 = 0.30487649791575028312e-3_f64 * t35776;
    let t37849 = 0.89430439388620083049e-2_f64 * t35781;
    let t37850 = 0.3286404220903135089e-2_f64 * t35786;
    let t37860 = 0.2439011983326002265e-2_f64 * t35798;
    let t37866 = t4036 * t2265;
    let t37872 = 0.18292589874945016987e-2_f64 * t36330;
    let t37904 = t1347 * t2244;
    let t37964 = 0.13659505348792789029e1_f64 * t36504;
    (t37822, t37825, t37848, t37849, t37850, t37860, t37866, t37872, t37904, t37964)
}
