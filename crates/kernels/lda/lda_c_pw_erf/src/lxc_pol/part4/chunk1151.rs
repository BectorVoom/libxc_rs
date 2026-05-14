//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1151/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1151<F: Float>(t16952: F, t3859: F, t519: F, t6331: F, t511: F, t7016: F, t16917: F, t16919: F, t16921: F, t16923: F, t16925: F, t16926: F, t16931: F, t16933: F, t16936: F, t16938: F, t16942: F, t16944: F, t16947: F, t16950: F) -> (F, F, F, F) {
    let t16953 = 16.0 / 135.0 * t16952;
    let t16955 = t519 * t3859 * t6331;
    let t16956 = 32.0 / 135.0 * t16955;
    let t16957 = t511 * t7016;
    let t16958 = 8.0 / 45.0 * t16957;
    let t16959 = t16917 - t16919 + t16921 + t16923 + t16925 + t16926 + t16931 + t16933 - t16936 - t16938 - t16942 - t16944 - t16947 + t16950 - t16953 - t16956 - t16958;
    (t16953, t16956, t16958, t16959)
}
