//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1046/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1046(t5: f64, t124330: f64, t124364: f64, t112: f64, t120067: f64, t124293: f64, t1442: f64, t1459: f64, t1774: f64, t1983: f64, t24987: f64, t24990: f64, t26902: f64, t26906: f64, t26969: f64, t27147: f64, t27171: f64, t31055: f64, t31057: f64, t31060: f64, t32108: f64, t32110: f64, t32197: f64, t32206: f64, t33878: f64, t35233: f64, t4028: f64, t510: f64, t5107: f64, t6876: f64, t7042: f64, t7057: f64, t8607: f64, t8718: f64, t8809: f64, t9003: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t124366 = piecewise3(t8, 0.0_f64, t124330 + t124364);
    let t124367 = t124366 * t112;
    let t124383 = -2.0_f64 * t124293 * t1459 - 4.0_f64 * t9003 * t27171 - t24987 * t8809 - 2.0_f64 * t8607 * t26902 - t31055 - t31057 - t31060 - t120067 - 4.0_f64 * t7042 * t27147 - 4.0_f64 * t35233 * t7057 - 2.0_f64 * t4028 * t32197 - t124367 * t510 - t1442 * t32108 + 3.0_f64 * t1983 * t32110 * t24990 + 6.0_f64 * t8607 * t26969 + 3.0_f64 * t6876 * t33878 + 6.0_f64 * t8607 * t26906 - 2.0_f64 * t32206 * t1774 - 2.0_f64 * t8718 * t5107;
    (t124367, t124383)
}
