//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1198/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1198<F: Float>(t10997: F, t10999: F, t11000: F, t11002: F, t11003: F, t11066: F, t11073: F, t14939: F, t14942: F, t14944: F, t14947: F, t8482: F, t8519: F, t8520: F, t8526: F, t8543: F) -> F {
    let t21720 = t8482 - t10997 - t8519 + F::cast_from(0.03253074390090522_f64) * t14939 - F::new(120.0) * t8520 - F::cast_from(1.7544670867903938_f64) * t14942 - F::cast_from(51.94757731704439_f64) * t14944 - F::cast_from(1.7544670867903938_f64) * t14947 + t8526 + t10999 + t11000 + t11002 - t11003 + F::new(60.0) * t8543 - t11066 - t11073;
    t21720
}
