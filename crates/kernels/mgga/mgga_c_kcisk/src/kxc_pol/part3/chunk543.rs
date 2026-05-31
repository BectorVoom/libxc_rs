//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 543/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk543<F: Float>(t4551: F, t4564: F, t1611: F, t1620: F, t240: F, t4164: F, t4167: F, t4173: F, t4322: F, t4528: F, t4530: F, t4535: F, t4536: F, t555: F) -> (F, F) {
    let t4565 = t4551 + t4564;
    let t4569 = t4164 - t4167 + t4173 - t4322 + t240 * (-t1611 * t4565 - F::cast_from(2.0_f64) * t1620 * t4530 + t4528 * t555 + F::cast_from(2.0_f64) * t4535 * t4536 - t4164 + t4167 - t4173 + t4322);
    (t4565, t4569)
}
