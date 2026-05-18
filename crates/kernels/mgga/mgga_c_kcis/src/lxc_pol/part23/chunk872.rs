//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 872/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk872<F: Float>(t16426: F, t3786: F, t1444: F, t1897: F, t2642: F, t3761: F, t1098: F, t5483: F, t1517: F, t531: F, t545: F, t1992: F, t3251: F) -> (F, F, F, F, F) {
    let t16427 = t3786 * t16426;
    let t16432 = t3761 * t1897 * t1444 * t2642;
    let t16436 = F::new(0.19711289e-2) * t1098 * t5483;
    let t16438 = t1517 * t545 * t531;
    let t16441 = t3251 * t1992;
    (t16427, t16432, t16436, t16438, t16441)
}
