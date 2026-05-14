//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 921/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk921<F: Float>(t16189: F, t16234: F, t16269: F, t16293: F, t1809: F, t5101: F, t1060: F, t2487: F, t1824: F, t1814: F, t5089: F, t6667: F, t10696: F, t2394: F, t10699: F, t4762: F) -> (F, F, F, F, F, F, F) {
    let t16295 = t16189 + t16234 + t16269 + t16293;
    let t16298 = t1809 * t5101;
    let t16299 = t2487 * t1060;
    let t16300 = t16299 * t1824;
    let t16303 = t5089 * t1814;
    let t16304 = t6667 * t1824;
    let t16307 = t10696 * t2394;
    let t16308 = t10699 * t4762;
    (t16295, t16298, t16300, t16303, t16304, t16307, t16308)
}
