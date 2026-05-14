//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1033/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1033<F: Float>(t10469: F, t10472: F, t10500: F, t10515: F, t10517: F, t10529: F, t10541: F, t10551: F, t10559: F, t10574: F, t10598: F, t10603: F, t10607: F, t10610: F, t10612: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13983 = 16.0 / 135.0 * t10469;
    let t13984 = 16.0 / 45.0 * t10472;
    let t13985 = 8.0 / 135.0 * t10500;
    let t13986 = 16.0 / 45.0 * t10515;
    let t13987 = 8.0 / 45.0 * t10517;
    let t13988 = 64.0 / 243.0 * t10529;
    let t13989 = 16.0 / 15.0 * t10541;
    let t13990 = 8.0 / 45.0 * t10551;
    let t13991 = 64.0 / 243.0 * t10559;
    let t13992 = 8.0 / 15.0 * t10574;
    let t13993 = 16.0 / 45.0 * t10598;
    let t13994 = 8.0 / 135.0 * t10603;
    let t13995 = 8.0 / 81.0 * t10607;
    let t13996 = t13983 + t13984 + t13985 + t13986 + t13987 + t13988 + t13989 + t13990 + t13991 - t13992 - t13993 - t13994 - t13995;
    let t13998 = 16.0 / 27.0 * t10610;
    let t13999 = 8.0 / 45.0 * t10612;
    (t13983, t13984, t13985, t13986, t13987, t13988, t13989, t13990, t13991, t13992, t13993, t13994, t13995, t13996, t13998, t13999)
}
