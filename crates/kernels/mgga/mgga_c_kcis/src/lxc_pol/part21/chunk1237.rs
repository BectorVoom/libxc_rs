//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1237/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1237<F: Float>(t10995: F, t7771: F, t27042: F, t27055: F, t27014: F, t27023: F, t2193: F, t2196: F, t44682: F, t26982: F, t7784: F, t1014: F, t26840: F) -> (F, F, F, F, F, F, F) {
    let t92948 = t7771 * t10995;
    let t92951 = t27042 * t27055;
    let t92955 = t27014 * t27023;
    let t92958 = t27014 * t27055;
    let t92964 = F::new(0.12871334876543209877e-3) * t2193 * t44682 * t2196;
    let t92976 = t26982 * t7784;
    let t92981 = t1014 * t26840;
    (t92948, t92951, t92955, t92958, t92964, t92976, t92981)
}
