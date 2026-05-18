//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 618/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk618<F: Float>(t3245: F, t558: F, t1014: F, t1460: F, t1465: F, t551: F) -> (F, F, F, F) {
    let t4114 = t3245 * t558;
    let t4115 = F::new(0.55273148148148148147e-3) * t4114;
    let t4117 = t1014 * t1460;
    let t4121 = F::new(1.0) / t1465 / t551;
    (t4114, t4115, t4117, t4121)
}
