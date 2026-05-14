//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1328/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1328<F: Float>(t111533: F, t2068: F, t32683: F, t110817: F, t110907: F, t110912: F, t111201: F, t111524: F, t1152: F, t2071: F, t22173: F, t2351: F, t2709: F, t294: F, t32539: F, t33987: F, t3472: F, t3473: F, t5586: F, t559: F, t9575: F, t9895: F) -> (F,) {
    let t113294 = 2.0 * t111533;
    let t113307 = t2068 * t32683;
    let t113308 = -t110817 + t111524 - t2709 * t22173 * t559 / 16.0 + t110907 + t1152 * t33987 / 8.0 - t110912 + t113294 - t294 * t2071 * t32539 / 16.0 - t294 * t3473 * t9895 / 16.0 - t2709 * t3472 * t2351 / 16.0 - t111201 - t294 * t5586 * t9575 / 8.0 + t113307;
    (t113308,)
}
