//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1347/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1347<F: Float>(t10039: F, t116067: F, t116069: F, t116072: F, t116074: F, t116077: F, t116080: F, t116083: F, t116085: F, t116087: F, t116089: F, t116091: F, t116094: F, t117460: F, t117479: F, t117499: F, t117518: F, t12342: F, t18923: F, t2042: F, t2049: F, t2815: F, t34650: F, t5532: F, t5552: F, t63011: F) -> (F,) {
    let t117526 = -t63011 * t2815 - t116067 + 2.0 * t5532 * t2815 * t18923 - t116069 - t116072 + t116074 - t116077 + t116080 + 4.0 * t5532 * t34650 * t2049 - t2042 * (t117460 + t117479 + t117499 + t117518) + t116083 + t116085 - t116087 - t12342 * t10039 - t116089 + 2.0 * t5532 * t10039 * t5552 - t116091 - t116094;
    (t117526,)
}
