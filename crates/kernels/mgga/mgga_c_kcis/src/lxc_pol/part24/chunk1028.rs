//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1028/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1028<F: Float>(t26717: F, t7690: F, t1014: F, t7723: F, t2180: F, t3245: F, t7732: F, t7727: F, t110: F, t2174: F, t2173: F, t7687: F, t7699: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26814 = t7690 * t26717;
    let t26826 = t1014 * t7723;
    let t26836 = t3245 * t2180;
    let t26837 = F::new(0.55273148148148148147e-3) * t26836;
    let t26838 = t1014 * t7732;
    let t26846 = t1014 * t7727;
    let t26854 = t110 * t2174;
    let t26856 = F::new(0.15445601851851851852e-3) * t2173 * t26854;
    let t26860 = t7687 * t7699;
    (t26814, t26826, t26836, t26837, t26838, t26846, t26854, t26856, t26860)
}
