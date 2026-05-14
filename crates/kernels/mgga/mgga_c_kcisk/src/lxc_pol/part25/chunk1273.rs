//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1273/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1273<F: Float>(t34663: F, t34665: F, t109134: F, t109135: F, t109136: F, t109141: F, t109144: F, t113273: F, t116053: F, t116054: F, t116055: F, t116056: F, t116057: F, t32872: F, t32875: F, t32878: F, t33322: F) -> (F,) {
    let t116058 = t34663 / 8.0;
    let t116059 = t34665 / 8.0;
    let t116060 = t116053 - t116054 - t116055 + t113273 - t32878 - t116056 - t109134 + t109135 - t109136 - t32872 + t109141 + t116057 - t116058 + t109144 + t116059 - t32875 - t33322;
    (t116060,)
}
