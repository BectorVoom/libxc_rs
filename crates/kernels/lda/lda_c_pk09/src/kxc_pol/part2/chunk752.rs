//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 752/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk752<F: Float>(t3820: F, t7731: F, t7608: F, t1067: F, t2222: F, t4705: F, t3823: F, t4581: F, t4612: F, t4614: F, t709: F, t7578: F, t7590: F, t7598: F, t7602: F, t8651: F) -> (F,) {
    let t8657 = t3820 * t7731;
    let t8663 = t3820 * t7608;
    let t8669 = t2222 * t1067;
    let t8675 = t4705 * t7608;
    let t8677 = -19.489173774580152 * t8651 * t709 - 3.7610742193750633 * t8657 - 7.5221484387501265 * t3823 * t7598 - 3.7610742193750633 * t3823 * t7602 - 3.7610742193750633 * t8663 - 3.7610742193750633 * t3823 * t7590 - 7.5221484387501265 * t3823 * t7578 + 1.9882715304939877 * t8669 + 37.27051603526593 * t4581 * t7598 + 18.635258017632964 * t4581 * t7602 + 18.635258017632964 * t8675 + t4612 + t4614;
    (t8677,)
}
