//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1337/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1337<F: Float>(t2568: F, t5273: F, t1930: F, t2454: F, t9709: F, t112121: F, t9977: F, t33109: F, t34345: F, t33103: F, t34313: F, t112102: F, t17875: F, t34329: F, t11226: F, t33120: F) -> (F, F, F, F, F, F, F, F) {
    let t117347 = t5273 * t2568;
    let t117349 = t1930 * t2454;
    let t117350 = t117349 * t9709;
    let t117352 = t112121 * t9977;
    let t117354 = t34345 * t33109;
    let t117356 = t34313 * t33103;
    let t117358 = t112102 * t17875;
    let t117360 = t34329 * t33103;
    let t117362 = t11226 * t33120;
    (t117347, t117350, t117352, t117354, t117356, t117358, t117360, t117362)
}
