//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1158/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1158<F: Float>(t6981: F, t9679: F, t1799: F, t4998: F, t9935: F, t9664: F, t1693: F, t2447: F, t1772: F) -> (F, F, F, F, F, F) {
    let t34115 = t9679 * t6981;
    let t34116 = t1799 * t34115;
    let t34118 = t4998 * t9935;
    let t34119 = t9664 * t34118;
    let t34121 = t1693 * t2447;
    let t34122 = t34121 * t1772;
    (t34115, t34116, t34118, t34119, t34121, t34122)
}
