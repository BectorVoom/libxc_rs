//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 697/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk697<F: Float>(t7921: F, t2260: F, t7926: F, t7929: F, t7933: F, t7936: F, t7964: F, t7968: F, t7971: F, t7976: F, t7978: F, t7981: F, t7986: F) -> (F, F) {
    let t7991 = F::new(0.11607361111111111111e-2) * t7921;
    let t7996 = -F::new(0.34752604166666666667e-3) * t7964 * t2260 + F::new(0.46377350260416666667e-4) * t7968 * t7971 - t7976 - F::new(0.11584201388888888889e-3) * t7978 * t7981 + F::new(0.34752604166666666667e-3) * t7978 * t7986 + F::new(0.34752604166666666667e-3) * t7978 * t7971 + t7991 + F::new(0.11607361111111111111e-2) * t7926 + F::new(0.17411041666666666666e-2) * t7929 - F::new(0.17411041666666666666e-2) * t7933 + F::new(0.11607361111111111111e-2) * t7936;
    (t7991, t7996)
}
