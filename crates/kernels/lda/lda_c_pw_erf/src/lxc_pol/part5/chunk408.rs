//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 408/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk408<F: Float>(t1074: F, t1060: F, t1001: F, t1036: F, t1040: F, t1053: F, t1057: F, t1079: F, t1083: F, t1087: F, t1802: F, t1805: F, t997: F) -> F {
    let t1888 = F::new(0.0001831155503675316) * t1074;
    let t1889 = F::new(0.5848223397455204) * t1060;
    let t1890 = -t1802 + t1040 - t997 + t1036 + t1805 - t1888 + t1083 - t1053 - t1057 - t1889 + t1079 + t1087 - t1001;
    t1890
}
