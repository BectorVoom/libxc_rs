//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1202/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1202<F: Float>(t32459: F, t8322: F, t32458: F, t2059: F, t2331: F, t32465: F, t32464: F) -> (F, F, F, F) {
    let t34949 = t32459 * t8322;
    let t34950 = t32458 * t34949;
    let t34954 = t32465 * t2059 * t2331;
    let t34955 = t32464 * t34954;
    (t34949, t34950, t34954, t34955)
}
