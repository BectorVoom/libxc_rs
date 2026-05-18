//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1186/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1186<F: Float>(t10429: F, t1358: F, t2299: F, t488: F, t2268: F, t27102: F, t6316: F, t10249: F, t6313: F, t31590: F, t426: F, t535: F) -> (F, F, F, F) {
    let t31998 = F::new(0.63233348079280332442e-2) * t1358 * t2299 * t10429 * t488;
    let t32001 = F::new(0.14227503317838074799e1) * t2268 * t6316 * t27102;
    let t32003 = F::new(0.91056021234163678716e0) * t6313 * t10249;
    let t32005 = t31590 * t426;
    let t32008 = F::new(0.56910013271352299198e-1) * t2268 * t535 * t32005;
    (t31998, t32001, t32003, t32008)
}
