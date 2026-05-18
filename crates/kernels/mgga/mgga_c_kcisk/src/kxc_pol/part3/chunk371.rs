//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 371/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk371<F: Float>(t1636: F, t1856: F, t1060: F, t158: F, t165: F, t173: F, t1809: F, t1829: F, t1834: F, t1836: F, t1841: F, t1843: F, t1847: F, t1850: F, t1855: F) -> (F, F) {
    let t1857 = t1856 * t1636;
    let t1860 = t1829 + F::new(0.11955719325063177623e-1) * t1809 * t1060 - t1834 - F::new(0.3513e-2) * t158 * t1836 + t1841 + F::new(0.7925e-3) * t165 * t1843 - t1847 - F::new(0.5179538907796306876e-4) * t1850 * t1060 + t1855 + F::new(0.50413125e-5) * t173 * t1857;
    (t1857, t1860)
}
