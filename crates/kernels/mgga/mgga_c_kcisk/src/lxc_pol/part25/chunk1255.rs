//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1255/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1255<F: Float>(t32935: F, t5014: F, t10886: F, t32951: F, t9664: F, t10879: F, t9666: F, t11204: F, t1772: F, t648: F, t44407: F, t662: F, t33028: F, t4811: F, t46925: F, t11245: F, t1763: F) -> (F, F, F, F, F, F, F, F) {
    let t112192 = t5014 * t32935;
    let t112209 = t9664 * t10886 * t32951;
    let t112212 = t9664 * t10879 * t9666;
    let t112216 = t11204 * t648 * t1772;
    let t112221 = t662 * t44407;
    let t112226 = t4811 * t33028;
    let t112236 = t46925 * t648 * t1772;
    let t112240 = t11245 * t1763 * t1772;
    (t112192, t112209, t112212, t112216, t112221, t112226, t112236, t112240)
}
