//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1014/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1014<F: Float>(t11750: F, t11790: F, t11825: F, t11885: F, t185: F, t186: F, t530: F, t9220: F, t1508: F, t2100: F, t9231: F, t1524: F, t2067: F) -> (F, F, F, F, F) {
    let t11891 = F::new(2.0) / F::new(15.0) * t185 * t186 * t530 * (t11750 + t11790 + t11825 + t11885);
    let t11892 = F::new(8.0) / F::new(15.0) * t9220;
    let t11894 = F::new(2.0) / F::new(5.0) * t1508 * t2100;
    let t11895 = F::new(4.0) / F::new(45.0) * t9231;
    let t11897 = F::new(4.0) / F::new(5.0) * t1524 * t2067;
    (t11891, t11892, t11894, t11895, t11897)
}
