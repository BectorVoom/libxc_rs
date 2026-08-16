//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1416/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1416(t1569: f64, t2769: f64, t786: f64, t10985: f64, t15017: f64, t1580: f64, t41117: f64, t1565: f64, t40781: f64, t40488: f64, t4354: f64, t268: f64, t40452: f64, t4371: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50208 = t786 * t1569 * t2769;
    let t50214 = t15017 * t10985;
    let t50248 = t41117 * t1580;
    let t50370 = t40781 * t1565;
    let t50372 = t40488 * t4354;
    let t50377 = t40452 * t4371 * t268;
    (t50208, t50214, t50248, t50370, t50372, t50377)
}
