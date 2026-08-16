//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1753/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1753(t1145: f64, t141: f64, t89853: f64, t12254: f64, t89822: f64, t68255: f64, t68257: f64, t81156: f64, t81158: f64, t89839: f64, t89851: f64, t89865: f64, t89869: f64, t89873: f64, t89877: f64, t90379: f64, t90384: f64) -> (f64, f64, f64) {
    let t90387 = t141 * t1145 * t89853;
    let t90390 = t141 * t12254 * t89822;
    let t90400 = 0.44152e0_f64 * t90379 + 0.80513333333333333336e0_f64 * t68255 - 0.53675555555555555556e0_f64 * t68257 + 0.298026e1_f64 * t90384 + 0.66228e0_f64 * t90387 + 0.22076e0_f64 * t90390 + 0.80513333333333333333e0_f64 * t81156 - 0.24154e1_f64 * t81158 - 0.60384999999999999999e0_f64 * t89839 + 0.181155e1_f64 * t89851 + 0.40256666666666666666e1_f64 * t89865 - 0.72462e1_f64 * t89869 + 0.72462e1_f64 * t89873 + 0.301925e0_f64 * t89877;
    (t90387, t90390, t90400)
}
