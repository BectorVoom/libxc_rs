//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1256/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1256<F: Float>(t3245: F, t8176: F, t27345: F, t8144: F, t1014: F, t28409: F, t97997: F, t27563: F, t28727: F, t28853: F, t1598: F, t251: F, t54624: F) -> (F, F, F, F, F, F, F) {
    let t98942 = t3245 * t8176;
    let t98945 = F::cast_from(0.46336805555555555556e-3_f64) * t8144 * t27345;
    let t98946 = t1014 * t28409;
    let t98978 = F::cast_from(0.15476481481481481481e-2_f64) * t97997;
    let t98986 = F::cast_from(0.61782407407407407408e-3_f64) * t28727 * t27563;
    let t98988 = F::cast_from(0.82448622685185185186e-4_f64) * t28853 * t27563;
    let t98994 = t54624 * t251 * t1598;
    (t98942, t98945, t98946, t98978, t98986, t98988, t98994)
}
