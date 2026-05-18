//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1153/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1153<F: Float>(t15852: F, t739: F, t1326: F, t519: F, t34: F, t6330: F, t4829: F, t1446: F, t7698: F, t15867: F, t1991: F, t21159: F, t21161: F, t21165: F, t21169: F, t21173: F, t21175: F, t21179: F, t21183: F, t21185: F) -> (F, F, F, F, F, F, F, F) {
    let t21186 = t15852 * t739;
    let t21189 = F::new(8.0) / F::new(15.0) * t519 * t1326 * t21186;
    let t21190 = t6330 * t34;
    let t21193 = F::new(16.0) / F::new(15.0) * t519 * t4829 * t21190;
    let t21195 = F::new(4.0) / F::new(9.0) * t1446 * t7698;
    let t21196 = t15867 * t739;
    let t21199 = F::new(4.0) / F::new(9.0) * t519 * t1991 * t21196;
    let t21200 = -t21159 - t21161 - t21165 - t21169 + t21173 - t21175 - t21179 - t21183 - t21185 - t21189 - t21193 + t21195 + t21199;
    (t21186, t21189, t21190, t21193, t21195, t21196, t21199, t21200)
}
