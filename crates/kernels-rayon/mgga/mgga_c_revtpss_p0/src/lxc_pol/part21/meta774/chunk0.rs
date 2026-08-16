//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2750/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2750(t14671: f64, t14686: f64, t14931: f64, t2724: f64, t10811: f64, t14707: f64, t14874: f64, t14673: f64, t40731: f64, t40593: f64, t4447: f64, t4462: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50598 = t14931 * t14686 * t14671 * t2724;
    let t50600 = t10811 * t14707;
    let t50602 = t10811 * t14874;
    let t50604 = t40731 * t14673;
    let t50605 = 0.16262400898971305032e-2_f64 * t50604;
    let t50606 = t40593 * t4447;
    let t50607 = 0.17006693853500995666e-1_f64 * t50606;
    let t50608 = t40593 * t4462;
    (t50598, t50600, t50602, t50605, t50607, t50608)
}
