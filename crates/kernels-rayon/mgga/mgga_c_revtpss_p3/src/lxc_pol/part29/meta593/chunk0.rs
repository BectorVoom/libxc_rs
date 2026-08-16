//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1977/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1977(t98141: f64, t98144: f64, t98146: f64, t98148: f64, t98152: f64, t98156: f64, t94424: f64, t94430: f64, t94444: f64, t94449: f64, t98135: f64, t98154: f64) -> f64 {
    let t102486 = 0.30488190661738479625e-3_f64 * t98141;
    let t102487 = 0.57165357490759649296e-4_f64 * t98144;
    let t102488 = 0.32012600194825403606e-1_f64 * t98146;
    let t102489 = 0.2168320119862840671e-2_f64 * t98148;
    let t102490 = 0.11433071498151929859e-2_f64 * t98152;
    let t102492 = 0.4065600224742826258e-3_f64 * t98156;
    let t102493 = 0.68598428988911579156e-2_f64 * t98135 + 0.4065600224742826258e-3_f64 * t94424 - 0.32012600194825403606e-1_f64 * t94430 + 0.43366402397256813418e-2_f64 * t94444 + 0.28582678745379824648e-4_f64 * t94449 - t102486 + t102487 + t102488 + t102489 - t102490 - 0.34299214494455789578e-2_f64 * t98154 - t102492;
    t102493
}
