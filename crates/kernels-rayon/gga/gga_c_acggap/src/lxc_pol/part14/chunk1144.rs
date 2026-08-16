//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1144/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1144(t5623: f64, t7561: f64, t5991: f64, t7822: f64, t5986: f64, t5981: f64, t1881: f64, t7605: f64, t142: f64, t6319: f64, t8888: f64, t2060: f64, t6293: f64, t7815: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39811 = t7561 * t5623;
    let t39813 = t7822 * t5991;
    let t39815 = t7822 * t5986;
    let t39817 = t7822 * t5981;
    let t39819 = t7605 * t1881;
    let t39822 = t8888 * t142 * t6319;
    let t39825 = t2060 * t7815 * t6293;
    (t39811, t39813, t39815, t39817, t39819, t39822, t39825)
}
