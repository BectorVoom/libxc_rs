//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 706/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk706<F: Float>(t1948: F, t6822: F, t142: F, t6586: F, t551: F, t6517: F, t1665: F, t1972: F, t1672: F, t1982: F, t6319: F, t6325: F) -> (F, F, F, F, F, F, F) {
    let t6823 = t1948 * t6822;
    let t6825 = t6586 * t142;
    let t6827 = t6825 * t551 * t6517;
    let t6829 = t1972 * t1665;
    let t6831 = t1982 * t1672;
    let t6836 = F::new(0.9421211958699838) * t6319;
    let t6838 = F::new(0.6280807972466558) * t6325;
    (t6823, t6825, t6827, t6829, t6831, t6836, t6838)
}
