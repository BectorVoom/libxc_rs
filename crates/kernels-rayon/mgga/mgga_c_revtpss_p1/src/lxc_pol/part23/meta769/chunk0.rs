//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2569/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2569(t1284: f64, t17288: f64, t3624: f64, t1260: f64, t17289: f64, t13032: f64, t17524: f64, t12881: f64, t5381: f64, t17861: f64, t17416: f64, t3647: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57040 = t17288 * t1284 * t3624;
    let t57053 = t17289 * t1260;
    let t57056 = t13032 * t17524;
    let t57094 = t5381 * t12881;
    let t57100 = t17861 * t3624;
    let t57118 = t3647 * t17416;
    (t57040, t57053, t57056, t57094, t57100, t57118)
}
