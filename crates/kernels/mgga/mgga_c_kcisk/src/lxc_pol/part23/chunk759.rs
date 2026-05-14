//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 759/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk759<F: Float>(t1155: F, t559: F, t1624: F, t296: F, t1156: F, t2752: F, t2714: F, t3491: F, t388: F, t3930: F) -> (F, F, F, F, F) {
    let t9411 = t1155 * t559;
    let t9414 = t296 * t1624;
    let t9419 = t1156 * t2752;
    let t9422 = t3491 * t2714;
    let t9425 = t3930 * t388;
    (t9411, t9414, t9419, t9422, t9425)
}
