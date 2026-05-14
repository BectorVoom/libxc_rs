//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 711/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk711<F: Float>(t12184: F, t1559: F, t8654: F, t3614: F, t8675: F, t2281: F, t3653: F, t637: F, t643: F, t1073: F, t2282: F, t8618: F, t632: F, t72: F, t1075: F, t8640: F) -> (F, F, F, F, F, F) {
    let t12186 = t8654 * t12184 * t1559;
    let t12190 = 2.0 / 27.0 * t8675 * t3614;
    let t12191 = t2281 * t3653;
    let t12193 = t637 * t12191 * t643;
    let t12198 = t637 * t8618 * t1073 * t2282;
    let t12201 = t72 * t632;
    let t12204 = t8640 * t1075;
    (t12186, t12190, t12193, t12198, t12201, t12204)
}
