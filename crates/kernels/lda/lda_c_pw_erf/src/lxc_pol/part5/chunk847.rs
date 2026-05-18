//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 847/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk847<F: Float>(t6285: F, t743: F, t1308: F, t571: F, t2385: F, t4763: F, t6275: F, t1319: F, t1318: F, t2146: F, t2389: F, t225: F, t7337: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7815 = t6285 * t743;
    let t7816 = t1308 * t7815;
    let t7818 = F::new(4.0) / F::new(15.0) * t571 * t7816;
    let t7820 = F::new(16.0) / F::new(15.0) * t4763 * t2385;
    let t7821 = t6275 * t743;
    let t7822 = t1319 * t7821;
    let t7824 = F::new(8.0) / F::new(15.0) * t1318 * t7822;
    let t7826 = F::new(8.0) / F::new(15.0) * t2146 * t2389;
    let t7827 = t7337 * t225;
    (t7815, t7816, t7818, t7820, t7821, t7822, t7824, t7826, t7827)
}
