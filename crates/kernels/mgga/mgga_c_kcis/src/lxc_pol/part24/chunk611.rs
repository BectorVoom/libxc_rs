//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 611/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk611<F: Float>(t1001: F, t6539: F, t286: F, t285: F, t2879: F, t4937: F, t4959: F, t6518: F, t6522: F, t6526: F, t6530: F, t6535: F, t991: F) -> (F, F, F) {
    let t6540 = t1001 * t6539;
    let t6541 = t286 * t6540;
    let t6544 = -t2879 + t4937 / F::new(432.0) - t4959 / F::new(144.0) + t991 * t6518 / F::new(216.0) - t991 * t6522 / F::new(144.0) - t991 * t6526 / F::new(144.0) + t991 * t6530 / F::new(288.0) + t285 * t6535 / F::new(48.0) - t285 * t6541 / F::new(96.0);
    (t6540, t6541, t6544)
}
