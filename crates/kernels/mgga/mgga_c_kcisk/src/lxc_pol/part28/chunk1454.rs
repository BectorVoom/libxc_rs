//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1454/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1454<F: Float>(t109134: F, t109135: F, t109136: F, t109141: F, t109144: F, t113273: F, t116053: F, t116054: F, t116055: F, t116056: F, t116057: F, t116058: F, t120957: F, t120975: F, t123479: F, t123489: F, t35059: F, t35078: F, t35080: F, t35535: F, t35538: F, t35541: F, t35544: F, t8: F) -> (F,) {
    let t123493 = t116053 - t116054 - t35078 - t116055 + t113273 + t8 * (t120957 + t120975 + t123479 + t123489) + t35059 - t35080 - t35535 - t116056 - t109134 + t109135 - t109136 - t35538 - t35541 + t109141 - t35544 + t116057 - t116058 + t109144;
    (t123493,)
}
