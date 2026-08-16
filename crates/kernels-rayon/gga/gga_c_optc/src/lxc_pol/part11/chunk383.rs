//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 383/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk383(t1849: f64, t1867: f64, t1761: f64, t1772: f64, t1787: f64, t1795: f64, t1803: f64, t1809: f64, t1811: f64, t1821: f64, t1826: f64, t1829: f64, t1834: f64, t1838: f64, t1842: f64, t1848: f64, t1850: f64, t1860: f64, t1865: f64, t209: f64, t4: f64, t566: f64, t573: f64, t581: f64, t588: f64, t71: f64, t84: f64) -> (f64, f64) {
    let t1868 = t1849 * t1867;
    let t1871 = -0.70981924444444444442e-3_f64 * t4 * t1772 * t71 - 0.34246666666666666666e-1_f64 * t209 * t1803 * t573 - 2.0_f64 * t1809 * t1811 + 1.0_f64 * t566 * t1821 + 0.32164683177870697974e2_f64 * t1826 * t1829 + t1834 + t1838 + t1761 - t1787 - t1795 - 0.24415406715670879921e-3_f64 * t4 * t1772 * t84 - 0.10843580882781524214e-1_f64 * t209 * t1842 * t588 - 0.11696446794910408142e1_f64 * t1848 * t1850 + 0.58482233974552040708e0_f64 * t581 * t1860 + 0.17315755899375863299e2_f64 * t1865 * t1868;
    (t1868, t1871)
}
