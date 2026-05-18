//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 826/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk826<F: Float>(t6944: F, t784: F, t1440: F, t1325: F, t2146: F, t2540: F, t2544: F, t6963: F, t811: F, t1466: F, t1318: F, t6979: F, t806: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7576 = t6944 * t784;
    let t7577 = t1440 * t7576;
    let t7579 = F::new(8.0) / F::new(5.0) * t1325 * t7577;
    let t7581 = F::new(4.0) / F::new(15.0) * t2146 * t2540;
    let t7583 = F::new(4.0) / F::new(9.0) * t2146 * t2544;
    let t7584 = t6963 * t811;
    let t7585 = t1466 * t7584;
    let t7587 = F::new(8.0) / F::new(5.0) * t1318 * t7585;
    let t7588 = t6979 * t806;
    (t7576, t7577, t7579, t7581, t7583, t7584, t7585, t7587, t7588)
}
