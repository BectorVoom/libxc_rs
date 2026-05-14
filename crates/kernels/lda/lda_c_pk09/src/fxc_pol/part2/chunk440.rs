//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 440/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk440<F: Float>(t1552: F, t1554: F, t1556: F, t1558: F, t2502: F, t2505: F, t2542: F, t2546: F, t300: F, t306: F, t2487: F, t304: F, t1571: F, t1573: F, t1575: F, t1577: F) -> (F, F, F, F, F) {
    let t2566 = t1552 - 1.4770435158815312 * t2542 + t1554 + 1.4770435158815312 * t2546 + t1556 - 0.2946275542389858 * t2502 + t1558 + 0.2946275542389858 * t2505;
    let t2567 = t300 * t2566;
    let t2568 = t2567 * t306;
    let t2571 = t304 * t2487;
    let t2578 = t1571 - 0.7661514025603425 * t2542 + t1573 + 0.7661514025603425 * t2546 + t1575 - 0.15282509383508946 * t2502 + t1577 + 0.15282509383508946 * t2505;
    (t2566, t2567, t2568, t2571, t2578)
}
