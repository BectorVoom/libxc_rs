//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 716/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk716<F: Float>(t3659: F, t920: F, t363: F, t4894: F, t4889: F, t4844: F, t5: F, t1080: F, t13273: F, t16586: F, t2240: F, t3601: F, t3660: F, t3665: F, t3668: F, t4890: F, t4895: F, t4898: F, t623: F, t650: F) -> (F,) {
    let t16591 = t3659 * t920;
    let t16594 = t4894 * t363;
    let t16601 = t4889 * t363;
    let t16612 = t5 * t4844;
    let t16615 = t623 * t16586 / 4.0 + t3601 * t3665 / 2.0 + t623 * t16591 / 2.0 + t623 * t16594 / 4.0 + t2240 * t4898 / 2.0 + t13273 * t1080 / 2.0 + t623 * t16601 / 4.0 + t2240 * t4895 / 4.0 + t3601 * t3660 / 2.0 + t3601 * t3668 / 2.0 + t2240 * t4890 / 4.0 + t16612 * t650 / 4.0;
    (t16615,)
}
