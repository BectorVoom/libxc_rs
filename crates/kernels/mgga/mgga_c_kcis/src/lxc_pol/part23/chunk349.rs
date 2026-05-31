//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 349/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk349<F: Float>(t1546: F, t2055: F, t487: F, t579: F, t488: F, t251: F) -> (F, F, F) {
    let t2056 = t1546 * t2055;
    let t2058 = t579 * t487;
    let t2060 = F::cast_from(1.0_f64) / t488 / t2058;
    let t2061 = t2060 * t251;
    (t2056, t2060, t2061)
}
