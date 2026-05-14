//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 618/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk618<F: Float>(t5671: F, t6175: F, t1322: F, t2075: F, t3937: F, t3936: F, t403: F) -> (F, F, F, F) {
    let t6176 = t6175 * t5671;
    let t6179 = t2075 * t1322;
    let t6180 = t3937 * t6179;
    let t6183 = t3936 * t403;
    (t6176, t6179, t6180, t6183)
}
