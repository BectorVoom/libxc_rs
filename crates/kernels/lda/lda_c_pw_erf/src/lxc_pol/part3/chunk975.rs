//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 975/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk975<F: Float>(t12987: F, t1318: F, t3899: F, t4942: F, t1466: F, t3667: F, t3669: F, t811: F, t10162: F, t1325: F, t2182: F, t2188: F, t3745: F, t10169: F, t10172: F, t10173: F, t12971: F, t12975: F, t12979: F, t12982: F, t12985: F) -> (F, F, F, F, F, F) {
    let t12988 = 64.0 / 45.0 * t12987;
    let t12990 = t1318 * t3899 * t4942;
    let t12991 = 16.0 / 15.0 * t12990;
    let t12996 = 8.0 / 5.0 * t1318 * t1466 * t3667 * t811 * t3669;
    let t12998 = t1325 * t10162 * t2182;
    let t12999 = 8.0 / 45.0 * t12998;
    let t13001 = 8.0 / 5.0 * t3745 * t2188;
    let t13002 = t12971 + 8.0 * t10169 - t10172 + 4.0 / 3.0 * t10173 - t12975 + t12979 - t12982 + t12985 + t12988 + t12991 - t12996 + t12999 + t13001;
    (t12988, t12991, t12996, t12999, t13001, t13002)
}
