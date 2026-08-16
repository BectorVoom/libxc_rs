//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1393/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1393(t1848: f64, t4562: f64, t20648: f64, t550: f64, t1284: f64, t6441: f64, t1276: f64, t13253: f64, t13292: f64, t1666: f64, t1673: f64, t1849: f64, t1856: f64, t19011: f64, t19050: f64, t20697: f64, t3413: f64, t4544: f64, t5942: f64, t5960: f64, t63114: f64, t63173: f64, t6442: f64) -> f64 {
    let t67868 = 2.0_f64 * t1848 * t4562;
    let t67874 = 2.0_f64 * t20648 * t550;
    let t67879 = 2.0_f64 * t6441 * t1284;
    let t67881 = 2.0_f64 * t1276 * t20697 + t13253 * t1856 + t13292 * t1849 + t1666 * t19050 + t1673 * t19011 + t3413 * t6442 + 2.0_f64 * t4544 * t5960 + 2.0_f64 * t4562 * t5942 + 2.0_f64 * t63114 + 2.0_f64 * t63173 + t67868 + t67874 + t67879;
    t67881
}
