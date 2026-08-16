//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 873/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk873(t11703: f64, t14236: f64, t14249: f64, t2067: f64, t14117: f64, t69839: f64, t8456: f64, t14116: f64, t14125: f64, t9170: f64, t21709: f64, t9152: f64) -> (f64, f64, f64, f64) {
    let t75615 = t14236 * t14249 * t2067 * t11703;
    let t75620 = t69839 * t14117 * t8456;
    let t75623 = t14116 * t14125 * t9170;
    let t75626 = t14116 * t21709 * t9152;
    (t75615, t75620, t75623, t75626)
}
