//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 385/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk385<F: Float>(t1814: F, t2487: F, t1835: F, t2364: F, t1842: F, t1856: F, t158: F, t165: F, t173: F, t1809: F, t1829: F, t1834: F, t1841: F, t1847: F, t1850: F, t1855: F, t2063: F) -> (F, F, F, F, F) {
    let t2488 = t1814 * t2487;
    let t2494 = t1835 * t2364;
    let t2497 = t1842 * t2364;
    let t2502 = t1856 * t2364;
    let t2505 = t1829 + F::new(0.11955719325063177623e-1) * t1809 * t2063 - t1834 - F::new(0.3513e-2) * t158 * t2494 + t1841 + F::new(0.7925e-3) * t165 * t2497 - t1847 - F::new(0.5179538907796306876e-4) * t1850 * t2063 + t1855 + F::new(0.50413125e-5) * t173 * t2502;
    (t2488, t2494, t2497, t2502, t2505)
}
