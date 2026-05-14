//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1342/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1342<F: Float>(t15204: F, t18696: F, t18697: F, t18698: F, t18702: F, t18703: F, t18704: F, t18705: F, t18706: F, t18707: F, t18709: F, t18711: F, t18713: F, t18714: F, t18716: F, t18718: F, t18719: F) -> (F,) {
    let t19333 = t18696 - 4.0 / 45.0 * t15204 + t18697 - t18698 - t18702 - t18703 - t18704 - t18705 + t18706 - t18707 - t18709 - t18711 + t18713 - t18714 - t18716 - t18718 + t18719;
    (t19333,)
}
