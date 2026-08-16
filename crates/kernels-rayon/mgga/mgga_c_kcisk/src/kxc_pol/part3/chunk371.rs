//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 371/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk371(t1636: f64, t1856: f64, t1060: f64, t158: f64, t165: f64, t173: f64, t1809: f64, t1829: f64, t1834: f64, t1836: f64, t1841: f64, t1843: f64, t1847: f64, t1850: f64, t1855: f64) -> (f64, f64) {
    let t1857 = t1856 * t1636;
    let t1860 = t1829 + 0.11955719325063177623e-1_f64 * t1809 * t1060 - t1834 - 0.3513e-2_f64 * t158 * t1836 + t1841 + 0.7925e-3_f64 * t165 * t1843 - t1847 - 0.5179538907796306876e-4_f64 * t1850 * t1060 + t1855 + 0.50413125e-5_f64 * t173 * t1857;
    (t1857, t1860)
}
