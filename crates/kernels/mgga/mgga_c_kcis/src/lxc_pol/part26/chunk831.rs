//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 831/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk831<F: Float>(t45: F, t5586: F, t1893: F, t3860: F, t1315: F, t5538: F, t3898: F, t13948: F, t5570: F, t1903: F, t2331: F, t5567: F, t659: F) -> (F, F, F, F, F, F, F) {
    let t16093 = t45 * t5586;
    let t16103 = t1893 * t3860;
    let t16115 = t5538 * t1315;
    let t16120 = t1893 * t3898;
    let t16127 = t13948 * t5570;
    let t16129 = t2331 * t1903;
    let t16144 = t659 * t5567;
    (t16093, t16103, t16115, t16120, t16127, t16129, t16144)
}
