//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 434/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk434(t1794: f64, t1796: f64, t1803: f64, t1806: f64, t1808: f64, t1812: f64, t1816: f64, t1825: f64, t1829: f64, t1833: f64, t1840: f64, t1844: f64, t1845: f64, t1847: f64, t1851: f64) -> f64 {
    let t1852 = -2.0_f64 * t1794 + 8.0_f64 * t1796 - t1803 - t1806 - t1808 + t1812 + t1816 + t1825 - t1829 - t1833 - t1840 + t1844 - 0.23392894490538584828e1_f64 * t1845 + 0.34631718211362927518e2_f64 * t1847 + t1851;
    t1852
}
