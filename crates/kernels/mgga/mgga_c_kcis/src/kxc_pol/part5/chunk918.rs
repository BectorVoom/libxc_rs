//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 918/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk918<F: Float>(t359: F, t9372: F, t9494: F, t3198: F, t4992: F, t86: F, t5168: F, t1018: F, t1747: F, t1017: F, sigma0: F) -> (F, F, F, F, F) {
    let t13131 = t359 * t9372;
    let t13155 = t359 * t9494;
    let t13172 = t86 * t4992 * t3198;
    let t13181 = t5168 * sigma0;
    let t13190 = t1018 * t1747;
    let t13192 = t86 * t1017 * t13190;
    (t13131, t13155, t13172, t13181, t13192)
}
