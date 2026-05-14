//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1065/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1065<F: Float>(t14360: F, t945: F, t321: F, t14175: F, t804: F, t2053: F, t1211: F, t21885: F, t8556: F, t14365: F, t14372: F, t14185: F, t2352: F, t353: F, t859: F, t20154: F, t3067: F, t4088: F, t938: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t52089 = t14360 * t945;
    let t52090 = t321 * t52089;
    let t52092 = t804 * t14175;
    let t52094 = t14360 * t2053;
    let t52105 = t1211 * t21885;
    let t52112 = t804 * t1211;
    let t52113 = t52112 * t8556;
    let t52115 = t321 * t14365;
    let t52127 = t321 * t14372;
    let t52131 = t859 * t353 * t14185 * t2352;
    let t52154 = t20154 * t3067 * t4088 * t938;
    (t52089, t52090, t52092, t52094, t52105, t52112, t52113, t52115, t52127, t52131, t52154)
}
