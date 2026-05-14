//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1243/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1243<F: Float>(t34345: F, t9977: F, t736: F, t9015: F, t7303: F, t8780: F, t9708: F, t5290: F, t8786: F, t22254: F, t748: F, t1800: F, t9079: F, t9082: F, t7316: F, t8939: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35309 = t34345 * t9977;
    let t35311 = t9015 * t736;
    let t35313 = t7303 * t8780;
    let t35314 = t9708 * t35313;
    let t35316 = t5290 * t8786;
    let t35317 = t9708 * t35316;
    let t35319 = t22254 * t748;
    let t35321 = t1800 * t9079;
    let t35323 = t1800 * t9082;
    let t35325 = t7316 * t8939;
    (t35309, t35311, t35313, t35314, t35316, t35317, t35319, t35321, t35323, t35325)
}
