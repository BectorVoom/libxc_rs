//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1304/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1304<F: Float>(t18431: F, t518: F, t1319: F, t6957: F, t1897: F, t5481: F, t6964: F, t1317: F, t7138: F, t21186: F, t21188: F, t21193: F, t21196: F, t21206: F, t21209: F, t21212: F, t21234: F, t21237: F, t21240: F, t21243: F) -> (F, F, F, F, F, F) {
    let t21534 = t518 * t18431;
    let t21537 = t6957 * t1319;
    let t21542 = t1897 * t5481;
    let t21551 = t6964 * t1319;
    let t21558 = t1317 * t7138;
    let t21581 = F::cast_from(0.91722222222222222223e-3_f64) * t21186 - F::cast_from(0.45861111111111111112e-2_f64) * t21237 + F::new(0.1651e-1) * t21234 + F::cast_from(0.11006666666666666667e-1_f64) * t21240 - F::cast_from(0.27516666666666666667e-2_f64) * t21188 - F::new(0.24765e-1) * t21243 - F::new(0.3302e-1) * t21206 + F::cast_from(0.13758333333333333333e-2_f64) * t21196 - F::cast_from(0.27516666666666666667e-2_f64) * t21209 + F::new(0.8255e-2) * t21212 - F::new(0.41275e-2) * t21193;
    (t21534, t21537, t21542, t21551, t21558, t21581)
}
