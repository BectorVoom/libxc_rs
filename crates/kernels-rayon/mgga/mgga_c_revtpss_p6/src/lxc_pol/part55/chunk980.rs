//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 980/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk980(t14230: f64, t28911: f64, t26304: f64, t27972: f64, t27864: f64, t1445: f64, t1904: f64, t2027: f64, t213: f64, t25930: f64, t26282: f64, t26365: f64, t26366: f64, t27868: f64, t28863: f64, t28890: f64, t28895: f64, t28897: f64, t28899: f64, t28903: f64, t28905: f64, t28909: f64, t561: f64, t5775: f64, t7295: f64, t7511: f64) -> f64 {
    let t28912 = t28911 * t14230;
    let t28915 = t26304 * t27972;
    let t28918 = t26304 * t27864;
    let t28923 = 0.8673628188205199462e0_f64 * t7295 * t28863 - 0.65854491829355115987e0_f64 * t26282 * t1904 - 0.4336814094102599731e0_f64 * t2027 * t28890 - t26365 + 0.72280234901709995518e-2_f64 * t26366 - 0.72280234901709995518e-2_f64 * t28895 + 0.12851425765524037203e-1_f64 * t28897 - 0.65854491829355115987e0_f64 * t28899 * t1445 + 0.54878743191129263322e-2_f64 * t28903 + 0.65854491829355115987e0_f64 * t213 * t28905 * t561 + 0.72280234901709995518e-2_f64 * t28909 - 0.8673628188205199462e0_f64 * t27868 * t28912 - 0.8673628188205199462e0_f64 * t25930 * t28915 - 0.8673628188205199462e0_f64 * t25930 * t28918 - 0.65854491829355115987e0_f64 * t7511 * t5775;
    t28923
}
