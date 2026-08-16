//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 873/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk873<F: Float>(t1013: F, t71: F, t420: F, t7195: F, t1008: F, t52: F, t7182: F, t1014: F, t2036: F, t23711: F, t23742: F, t23810: F, t23832: F, t23842: F, t23866: F, t32782: F, t32822: F, t34461: F, t34857: F, t34864: F, t34868: F, t34873: F, t7318: F, t8812: F, t8859: F) -> (F, F, F, F, F, F) {
    let t34876 = t71 * t1013;
    let t34877 = t420 * t34876;
    let t34878 = t7195 * t34877;
    let t34884 = t52 * t7182 * t1008;
    let t34888 = t52 * t7182 * t1013;
    let t34893 = F::cast_from(0.20527106943485609994e0_f64) * t8812 * t34857 - F::cast_from(0.10263553471742804997e0_f64) * t2036 * t7318 * t1014 - F::cast_from(0.82108427773942439976e0_f64) * t23866 * t34864 + F::cast_from(0.41054213886971219988e0_f64) * t8859 * t34868 - F::cast_from(0.18125821328051150223e0_f64) * t23832 * t34873 + F::cast_from(0.18125821328051150223e0_f64) * t23842 * t34878 - t32782 - F::cast_from(0.30209702213418583705e-1_f64) * t23711 * t34461 + F::cast_from(0.45306850413028723348e0_f64) * t32822 * t34884 - F::cast_from(0.22653425206514361674e0_f64) * t23742 * t34888 + F::cast_from(0.41054213886971219988e0_f64) * t23810 * t34864;
    (t34876, t34877, t34878, t34884, t34888, t34893)
}
