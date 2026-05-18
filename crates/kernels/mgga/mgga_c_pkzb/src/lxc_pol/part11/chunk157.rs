//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 157/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk157<F: Float>(t486: F, t50: F, t65: F, t477: F, t479: F, t483: F) -> (F, F, F) {
    let t487 = t50 * t486;
    let t488 = t65 * t487;
    let t490 = -F::new(0.632975e0) * t477 - F::new(0.29896666666666666667e0) * t479 - F::new(0.1023875e0) * t483 - F::new(0.82156666666666666667e-1) * t488;
    (t487, t488, t490)
}
