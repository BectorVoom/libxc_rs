//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1032/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1032<F: Float>(t37327: F, t4176: F, t42886: F, t14656: F, t986: F, t3270: F, t3269: F, t11479: F, t3579: F, t495: F, t797: F, t11518: F, t11629: F, t3262: F, t9560: F, t3275: F, t3276: F) -> (F, F, F, F, F) {
    let t42889 = 15.0 / 8.0 * t37327 * t4176 * t42886;
    let t42890 = t14656 * t986;
    let t42891 = t3270 * t42890;
    let t42893 = t3269 * t42891 / 2.0;
    let t42897 = t3579 * t495 * t11479 * t797 / 2.0;
    let t42900 = 15.0 / 8.0 * t3262 * t11629 * t11518;
    let t42901 = t797 * t9560;
    let t42904 = 5.0 / 16.0 * t3275 * t3276 * t42901;
    (t42889, t42893, t42897, t42900, t42904)
}
