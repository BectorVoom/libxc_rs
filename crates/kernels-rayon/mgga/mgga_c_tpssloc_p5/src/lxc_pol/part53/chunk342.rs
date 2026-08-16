//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 342/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk342(t1164: f64, t1703: f64, t1420: f64, t338: f64, t1178: f64, t1409: f64, t1177: f64, t1111: f64, t1668: f64, t457: f64, t460: f64, t974: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1705 = 0.5848223622634646207e0_f64 * t1164 * t1703;
    let t1706 = t1420 * t338;
    let t1709 = t1178 * t1409;
    let t1710 = t1177 * t1709;
    let t1714 = t1111 / 6.0_f64 - t1668 / 6.0_f64;
    let t1715 = t457 * t1714;
    let t1716 = t1715 * t460;
    let t1717 = t974 * t1716;
    (t1705, t1706, t1709, t1710, t1714, t1716, t1717)
}
