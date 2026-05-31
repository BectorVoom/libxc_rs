//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1162/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1162<F: Float>(t3269: F, t42891: F, t11479: F, t3579: F, t495: F, t797: F, t11518: F, t11629: F, t3262: F, t9560: F, t3275: F, t3276: F) -> (F, F, F, F) {
    let t42893 = t3269 * t42891 / F::cast_from(2.0_f64);
    let t42897 = t3579 * t495 * t11479 * t797 / F::cast_from(2.0_f64);
    let t42900 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t3262 * t11629 * t11518;
    let t42901 = t797 * t9560;
    let t42904 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t3275 * t3276 * t42901;
    (t42893, t42897, t42900, t42904)
}
