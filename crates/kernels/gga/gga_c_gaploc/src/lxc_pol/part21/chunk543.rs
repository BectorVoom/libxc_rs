//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 543/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk543<F: Float>(t1457: F, t2959: F, t2949: F, t723: F, t1445: F, t2936: F, t313: F) -> (F, F, F, F, F) {
    let t3015 = t1457 * t2959;
    let t3018 = t2949 * t723;
    let t3019 = t1445 * t3018;
    let t3022 = t1445 * t2959;
    let t3025 = t313 * t2936;
    (t3015, t3018, t3019, t3022, t3025)
}
