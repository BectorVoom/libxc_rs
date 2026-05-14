//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1124/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1124<F: Float>(t33167: F, t9740: F, t4648: F, t9741: F, t1775: F, t11986: F, t79: F) -> (F, F, F, F) {
    let t33168 = t9740 * t33167;
    let t33172 = t9741 * t4648;
    let t33173 = t1775 * t33172;
    let t33176 = t11986 * t79;
    (t33168, t33172, t33173, t33176)
}
