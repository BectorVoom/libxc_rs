//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1256/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1256<F: Float>(t14255: F, t14277: F, t12541: F, t826: F, t1318: F, t2065: F, t4892: F, t4893: F, t1446: F, t6682: F, t1480: F, t6205: F, t1488: F, t18652: F, t18654: F, t18656: F, t18658: F, t18659: F, t18660: F, t18661: F, t18662: F, t18663: F, t18664: F) -> (F, F, F, F, F, F, F, F) {
    let t18665 = 32.0 / 243.0 * t14255;
    let t18666 = 16.0 / 15.0 * t14277;
    let t18668 = 8.0 / 45.0 * t12541 * t826;
    let t18672 = 16.0 / 15.0 * t1318 * t4892 * t4893 * t2065;
    let t18673 = t1446 * t6682;
    let t18674 = 32.0 / 135.0 * t18673;
    let t18676 = 4.0 / 45.0 * t6205 * t1480;
    let t18678 = 4.0 / 27.0 * t6205 * t1488;
    let t18679 = -t18652 + t18654 - t18656 - t18658 - t18659 - t18660 - t18661 - t18662 + t18663 + t18664 - t18665 - t18666 + t18668 + t18672 - t18674 + t18676 + t18678;
    (t18665, t18666, t18668, t18672, t18674, t18676, t18678, t18679)
}
