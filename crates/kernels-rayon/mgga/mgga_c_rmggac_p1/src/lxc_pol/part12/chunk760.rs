//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 760/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk760(t35618: f64, t7490: f64, t7552: f64, t7558: f64, t290: f64, t35604: f64, t2010: f64, t7755: f64, t1341: f64, t303: f64, t638: f64, t7310: f64) -> (f64, f64, f64, f64, f64) {
    let t35619 = 0.91462949374725084942e-3_f64 * t35618;
    let t35620 = t7490 * t7552;
    let t35621 = t35620 * t7558;
    let t35622 = 0.13010691197123848594e-3_f64 * t35621;
    let t35623 = t290 * t35604;
    let t35625 = t2010 * t7755 * t35623;
    let t35629 = t638 * t7310 * t303 * t1341;
    (t35619, t35622, t35623, t35625, t35629)
}
