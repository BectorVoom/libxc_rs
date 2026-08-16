//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 944/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk944<F: Float>(t1346: F, t3835: F, t12924: F, t425: F, t13277: F, t1354: F, t1056: F, t3593: F, t3831: F, t3900: F, t960: F, t1376: F, t3114: F) -> (F, F, F, F, F, F) {
    let t13973 = t1346 * t3835;
    let t13975 = t425 * t12924;
    let t13978 = t1354 * t13277;
    let t13982 = t3831 * t1056 * t3593;
    let t13987 = t960 * t3900;
    let t13989 = t3114 * t1376;
    (t13973, t13975, t13978, t13982, t13987, t13989)
}
