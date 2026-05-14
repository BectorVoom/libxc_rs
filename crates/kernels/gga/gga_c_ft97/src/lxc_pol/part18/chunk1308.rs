//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1308/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1308<F: Float>(t1009: F, t1014: F, t104834: F, t105106: F, t105143: F, t105190: F, t105244: F, t105270: F, t12553: F, t23728: F, t23847: F, t23866: F, t26665: F, t26674: F, t26678: F, t5784: F, t5802: F, t6593: F, t6597: F, t7335: F, t8812: F, t8833: F, t94479: F, t94518: F, t94521: F, t94932: F, t94948: F) -> (F,) {
    let t105324 = -0.45306850413028723348e0 * t8833 * t105143 - 0.21895580739717983994e1 * t7335 * t105270 - 0.45306850413028723348e0 * t23847 * t105244 - 0.12220869211492952596e0 * t94518 * t1009 + 0.61104346057464762978e-1 * t94521 * t1014 - 0.45306850413028723348e0 * t12553 * t5784 * t6593 - 0.90613700826057446696e0 * t26674 * t26678 + 0.43791161479435967988e1 * t23866 * t105106 + 0.53706137268299704367e-1 * t94932 - 0.45306850413028723348e0 * t94479 * t6597 + 0.48327307107230638238e1 * t26674 * t26665 + 0.48327307107230638238e1 * t5802 * t104834 - 0.15303647250623035442e2 * t5802 * t105190 + 0.17780800291358024692e0 * t94948 + 0.93056218143801431978e1 * t8812 * t23728 * t1009;
    (t105324,)
}
