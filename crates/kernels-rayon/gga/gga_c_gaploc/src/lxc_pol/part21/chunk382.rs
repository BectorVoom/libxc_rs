//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 382/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk382(t1819: f64, t231: f64, t1163: f64, t643: f64, t4: f64, t1167: f64, t1074: f64, t646: f64, t1178: f64, t1186: f64, t1126: f64, t1131: f64, t1138: f64, t1153: f64, t1161: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1820 = t231 * t1819;
    let t1822 = 0.24415406715670879921e-3_f64 * t643 * t1163;
    let t1823 = t231 * t4;
    let t1825 = 0.10843580882781524214e-1_f64 * t1823 * t1167;
    let t1827 = 0.11696446794910408142e1_f64 * t646 * t1074;
    let t1829 = 0.58482233974552040708e0_f64 * t646 * t1178;
    let t1831 = 0.17315755899375863299e2_f64 * t646 * t1186;
    let t1832 = -t1126 - t1131 - t1138 + t1153 + t1161 + t1820 + t1822 + t1825 + t1827 - t1829 - t1831;
    (t1820, t1822, t1825, t1827, t1829, t1831, t1832)
}
