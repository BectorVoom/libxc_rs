//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1723/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1723(t30: f64, t265: f64, t393: f64, t87990: f64, t88042: f64, t88577: f64, t88603: f64, t89756: f64, t1468: f64, t1469: f64, t1587: f64, t1704: f64, t22670: f64, t22671: f64, t23436: f64, t24192: f64, t395: f64, t45: f64, t5824: f64, t5825: f64, t6084: f64, t6405: f64, t87125: f64, t87126: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> f64 {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t89759 = piecewise3(t394, t88042 + t88577 + t88603 + t89756, t87990);
    let t89771 = piecewise3(t120, t87990 * t30 / 2.0_f64 + 2.0_f64 * t23436 * t1468 + 3.0_f64 * t6084 * t5824 + 2.0_f64 * t1587 * t22670 + t265 * t87125 / 2.0_f64, t89759 * t45 / 2.0_f64 + 2.0_f64 * t24192 * t1469 + 3.0_f64 * t6405 * t5825 + 2.0_f64 * t1704 * t22671 + t395 * t87126 / 2.0_f64);
    t89771
}
