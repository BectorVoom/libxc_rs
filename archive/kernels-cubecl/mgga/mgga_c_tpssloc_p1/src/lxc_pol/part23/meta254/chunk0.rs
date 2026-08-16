//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 915/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk915<F: Float>(t3640: F, t6270: F, t11947: F, t6274: F, t5385: F, t604: F, t1409: F, t65: F, t67: F, t5392: F, t9287: F, t9300: F) -> (F, F, F, F, F, F) {
    let t19267 = t6270 * t3640;
    let t19270 = t6274 * t11947;
    let t19299 = t5385 * t604;
    let t19322 = t1409 * t65 * t67;
    let t19368 = t9287 * t5392;
    let t19390 = t9300 * t5392;
    (t19267, t19270, t19299, t19322, t19368, t19390)
}
