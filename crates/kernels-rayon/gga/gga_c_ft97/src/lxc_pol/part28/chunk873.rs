//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 873/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk873(t1013: f64, t71: f64, t420: f64, t7195: f64, t1008: f64, t52: f64, t7182: f64, t1014: f64, t2036: f64, t23711: f64, t23742: f64, t23810: f64, t23832: f64, t23842: f64, t23866: f64, t32782: f64, t32822: f64, t34461: f64, t34857: f64, t34864: f64, t34868: f64, t34873: f64, t7318: f64, t8812: f64, t8859: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34876 = t71 * t1013;
    let t34877 = t420 * t34876;
    let t34878 = t7195 * t34877;
    let t34884 = t52 * t7182 * t1008;
    let t34888 = t52 * t7182 * t1013;
    let t34893 = 0.20527106943485609994e0_f64 * t8812 * t34857 - 0.10263553471742804997e0_f64 * t2036 * t7318 * t1014 - 0.82108427773942439976e0_f64 * t23866 * t34864 + 0.41054213886971219988e0_f64 * t8859 * t34868 - 0.18125821328051150223e0_f64 * t23832 * t34873 + 0.18125821328051150223e0_f64 * t23842 * t34878 - t32782 - 0.30209702213418583705e-1_f64 * t23711 * t34461 + 0.45306850413028723348e0_f64 * t32822 * t34884 - 0.22653425206514361674e0_f64 * t23742 * t34888 + 0.41054213886971219988e0_f64 * t23810 * t34864;
    (t34876, t34877, t34878, t34884, t34888, t34893)
}
