//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1673/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1673(t14987: f64, t2467: f64, t122: f64, t4480: f64, t2466: f64, t10995: f64, t11044: f64, t4481: f64, t2435: f64, t4477: f64, t136: f64, t1579: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14989 = 0.19514881078765566038e-1_f64 * t14987 * t2467;
    let t14990 = t4480 * t122;
    let t14991 = t14990 * t2466;
    let t14992 = t10995 * t14991;
    let t14995 = 0.19514881078765566038e-1_f64 * t11044 * t4481;
    let t14998 = t2435 * t4477;
    let t15002 = t1579 * t136;
    (t14989, t14990, t14991, t14992, t14995, t14998, t15002)
}
