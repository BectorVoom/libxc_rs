//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 714/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk714<F: Float>(t12706: F, t10628: F, t2365: F, t6111: F, t10893: F, t959: F, t13079: F, t13098: F, t13102: F, t13106: F, t13110: F, t13113: F, t13114: F, t13115: F, t13116: F, t317: F, t797: F, t813: F, t833: F) -> (F, F) {
    let t13117 = F::new(0.63904876589867916127e-1) * t12706;
    let t13118 = t2365 * t10628;
    let t13119 = t6111 * t13118;
    let t13120 = F::new(0.59584149919750711116e-1) * t13119;
    let t13121 = t10893 * t959;
    let t13123 = t13079 + F::new(0.35750489951850426669e0) * t13098 * t317 - F::new(0.35750489951850426669e0) * t797 * t13102 - F::new(0.23005755572352449806e1) * t813 * t13106 + F::new(0.23005755572352449806e1) * t833 * t13110 - t13113 - t13114 + t13115 + t13116 + t13117 + t13120 + F::new(0.29792074959875355558e-1) * t13121;
    (t13118, t13123)
}
