//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1206/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1206(t102610: f64, t102629: f64, t102636: f64, t108282: f64, t109706: f64, t109715: f64, t109858: f64, t114640: f64, t114666: f64, t1904: f64, t25930: f64, t26304: f64, t27837: f64, t30071: f64, t30267: f64, t8095: f64, t8104: f64, t94823: f64, t96549: f64, t96564: f64, t96584: f64, t96591: f64) -> f64 {
    let t115258 = -0.32927245914677557992e-1_f64 * t109715 - 0.72280234901709995519e-3_f64 * t102610 - 0.13010442282307799193e1_f64 * t30071 * t8104 - 0.51405703062096148814e-2_f64 * t102629 - 0.72280234901709995519e-3_f64 * t102636 + 0.13010442282307799193e1_f64 * t27837 * t30267 + 0.78062653693846795158e1_f64 * t94823 * t26304 * t114666 - 0.26020884564615598386e1_f64 * t25930 * t26304 * t114640 + t96549 - 0.19756347548806534796e1_f64 * t109706 * t1904 - t96564 + 0.26020884564615598386e1_f64 * t108282 * t8095 - t96584 - 0.29272321618148349057e-1_f64 * t109858 + t96591;
    t115258
}
