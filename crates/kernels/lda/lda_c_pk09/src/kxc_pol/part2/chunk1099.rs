//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1099/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1099<F: Float>(t12099: F, t12113: F, t132: F, t11461: F, t11951: F, t12041: F, t12075: F, t12082: F, t1783: F, t1856: F, t2007: F, t2752: F, t455: F, t6928: F, t6930: F, t6954: F, t6956: F, t6958: F, t6962: F, t6964: F, t6969: F, t7335: F, t7337: F, t7469: F, t93: F) -> (F, F) {
    let t12114 = t12099 + t12113;
    let t12115 = t132 * t12114;
    let t12119 = t6928 + t6930 - t6954 + t6956 - t6958 - t6962 - F::new(1.2536914064583544) * t6964 + t6969 - t2007 * t11951 - F::new(2.2140749178833072) * t12041 * t455 - t7335 - t7337 - F::new(2.427516195194328) * t12075 * t455 + F::new(2.427516195194328) * t1856 * t11461 + F::new(2.427516195194328) * t7469 * t2752 - F::new(2.427516195194328) * t12082 * t455 - F::new(1.7770439370459628) * t1783 * t93 * t12115;
    (t12114, t12119)
}
