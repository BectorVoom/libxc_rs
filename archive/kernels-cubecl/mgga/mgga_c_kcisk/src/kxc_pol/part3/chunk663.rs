//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 663/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk663<F: Float>(t4798: F, t4817: F, t1869: F, t4805: F, t4811: F, t1865: F, t3805: F, t167: F, t3281: F, t10449: F, t8: F, t1899: F) -> (F, F, F, F, F, F, F) {
    let t10512 = t4817 * t4798;
    let t10513 = t1869 * t10512;
    let t10515 = t4811 * t4805;
    let t10517 = t3805 * t1865;
    let t10519 = F::cast_from(6.0_f64) * t167;
    let t10520 = F::cast_from(6.0_f64) * t3281;
    let t10522 = t10449 * t8 + t10519 - t10520;
    let t10523 = t1899 * t10522;
    (t10513, t10515, t10517, t10519, t10520, t10522, t10523)
}
