//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1189/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1189<F: Float>(t32045: F, t8171: F, t1339: F, t2075: F, t5874: F, t6183: F) -> (F, F, F, F) {
    let t34758 = t32045 * t8171;
    let t34759 = t1339 * t34758;
    let t34762 = t5874 * t2075;
    let t34763 = t6183 * t34762;
    (t34758, t34759, t34762, t34763)
}
