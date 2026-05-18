//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 827/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk827<F: Float>(t1440: F, t7588: F, t1325: F, t6988: F, t799: F, t2558: F, t4738: F, t6991: F, t833: F, t1466: F, t1318: F, t6997: F, t784: F) -> (F, F, F, F, F, F, F, F) {
    let t7589 = t1440 * t7588;
    let t7591 = F::new(4.0) / F::new(5.0) * t1325 * t7589;
    let t7593 = F::new(8.0) / F::new(15.0) * t6988 * t799;
    let t7595 = F::new(8.0) / F::new(5.0) * t4738 * t2558;
    let t7596 = t6991 * t833;
    let t7597 = t1466 * t7596;
    let t7599 = F::new(4.0) / F::new(5.0) * t1318 * t7597;
    let t7600 = t6997 * t784;
    (t7589, t7591, t7593, t7595, t7596, t7597, t7599, t7600)
}
