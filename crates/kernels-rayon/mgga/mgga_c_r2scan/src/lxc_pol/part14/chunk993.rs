//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 993/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk993(t2206: f64, t978: f64, t146: f64, t3305: f64, t10781: f64, t2553: f64, t10856: f64, t2842: f64, t10894: f64, t927: f64, t787: f64, t3320: f64, t783: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11747 = t2206 * t978;
    let t11748 = t146 * t11747;
    let t11749 = t11748 * t3305;
    let t11751 = t10781 * t2553;
    let t11753 = t10856 * t2842;
    let t11758 = t10894 * t927;
    let t11760 = t978 * t787;
    let t11762 = t783 * t11760 * t3320;
    (t11747, t11748, t11749, t11751, t11753, t11758, t11760, t11762)
}
