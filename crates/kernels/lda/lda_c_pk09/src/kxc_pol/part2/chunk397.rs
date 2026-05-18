//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 397/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk397<F: Float>(t1240: F, t514: F, t454: F, t1948: F, t1672: F, t498: F, t502: F, t490: F, t1729: F, t545: F, t537: F, t524: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1949 = t514 * t1240;
    let t1950 = t454 * t1949;
    let t1952 = F::new(0.04115066352984959) * t1948 * t1950;
    let t1954 = F::new(0.6268457032291772) * t498 * t1672;
    let t1956 = F::new(6.496391258193384) * t502 * t1672;
    let t1958 = F::new(1.2536914064583544) * t490 * t1672;
    let t1959 = t545 * t1729;
    let t1962 = t537 * t1729;
    let t1965 = t524 * t1729;
    (t1949, t1950, t1952, t1954, t1956, t1958, t1959, t1962, t1965)
}
