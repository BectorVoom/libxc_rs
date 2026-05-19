//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 390/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk390<F: Float>(t801: F, t798: F, t1932: F, t1938: F, t1942: F, t1946: F, t1951: F, t1955: F) -> (F, F, F, F) {
    let t2040 = t801 * t801;
    let t2041 = F::new(1.0) / t2040;
    let t2042 = t798 * t2041;
    let t2049 = F::new(0.9375e-1) * t1932 - F::new(0.9375e-1) * t1938 + F::new(0.625e-1) * t1942 - F::cast_from(0.101171875e-1_f64) * t1946 + F::cast_from(0.101171875e-1_f64) * t1951 - F::cast_from(0.13489583333333333333e-1_f64) * t1955;
    (t2040, t2041, t2042, t2049)
}
