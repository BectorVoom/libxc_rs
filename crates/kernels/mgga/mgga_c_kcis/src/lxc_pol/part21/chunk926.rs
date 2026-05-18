//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 926/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk926<F: Float>(t14067: F, t14068: F, t3200: F, t1022: F, t9409: F, t4818: F, t922: F, t2861: F, t4774: F, t4549: F, t9429: F, t4802: F) -> (F, F, F, F, F, F, F) {
    let t14069 = t14067 * t14068;
    let t14070 = t3200 * t14069;
    let t14072 = t9409 * t1022;
    let t14073 = t4818 * t922;
    let t14074 = t14072 * t14073;
    let t14075 = t3200 * t14074;
    let t14078 = t2861 * t4774;
    let t14079 = F::new(0.33163888888888888888e-2) * t14078;
    let t14081 = t9429 * t4549;
    let t14085 = t9429 * t4802;
    (t14070, t14073, t14075, t14078, t14079, t14081, t14085)
}
