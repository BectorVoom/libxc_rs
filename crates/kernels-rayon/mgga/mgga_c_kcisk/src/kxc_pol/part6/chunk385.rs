//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 385/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk385(t1814: f64, t2487: f64, t1835: f64, t2364: f64, t1842: f64, t1856: f64, t158: f64, t165: f64, t173: f64, t1809: f64, t1829: f64, t1834: f64, t1841: f64, t1847: f64, t1850: f64, t1855: f64, t2063: f64) -> (f64, f64, f64, f64, f64) {
    let t2488 = t1814 * t2487;
    let t2494 = t1835 * t2364;
    let t2497 = t1842 * t2364;
    let t2502 = t1856 * t2364;
    let t2505 = t1829 + 0.11955719325063177623e-1_f64 * t1809 * t2063 - t1834 - 0.3513e-2_f64 * t158 * t2494 + t1841 + 0.7925e-3_f64 * t165 * t2497 - t1847 - 0.5179538907796306876e-4_f64 * t1850 * t2063 + t1855 + 0.50413125e-5_f64 * t173 * t2502;
    (t2488, t2494, t2497, t2502, t2505)
}
