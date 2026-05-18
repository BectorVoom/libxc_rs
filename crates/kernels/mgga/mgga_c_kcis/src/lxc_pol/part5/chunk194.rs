//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 194/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk194<F: Float>(t453: F, t456: F, t459: F, t463: F) -> (F, F, F) {
    let t597 = F::new(0.705945e1) * t456 + F::new(0.1549425e1) * t453 + F::new(0.420775e0) * t459 + F::new(0.1562925e0) * t463;
    let t600 = F::new(1.0) + F::new(0.32164683177870697974e2) / t597;
    let t601 = f64::ln(t600);
    (t597, t600, t601)
}
