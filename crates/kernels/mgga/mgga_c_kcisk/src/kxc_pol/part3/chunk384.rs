//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 384/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk384<F: Float>(t1905: F, t1958: F, t2038: F, t2042: F, t2049: F, t240: F, t802: F, t116: F, t213: F, t172: F, t32: F, t5: F, t142: F, t814: F, t298: F, t831: F) -> (F, F, F, F, F) {
    let t2053 = t1905 - t1958 + t240 * (t2038 * t802 - t2042 * t2049 - t1905 + t1958);
    let t2689 = t116 * t213;
    let t2849 = 0.14764770444444444444e-2 * t5 * t172 * t32;
    let t2850 = t142 * t814;
    let t2853 = 0.35616666666666666667e-1 * t298 * t2850 * t831;
    (t2053, t2689, t2849, t2850, t2853)
}
