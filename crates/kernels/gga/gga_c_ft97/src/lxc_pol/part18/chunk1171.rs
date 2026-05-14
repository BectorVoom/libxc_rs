//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1171/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1171<F: Float>(t22619: F, t25653: F, t415: F, t1608: F, t17839: F, t5596: F, t35: F, t358: F, t363: F, t3366: F, t25759: F, t420: F, t3066: F, t379: F, t3188: F, t6449: F, t92557: F) -> (F, F, F, F, F, F, F) {
    let t100753 = 0.29693535778629056444e-3 * t22619 * t415 * t25653;
    let t100763 = t1608 * t5596 * t17839;
    let t100775 = t35 * t358;
    let t100776 = t100775 * t363;
    let t100777 = t3366 * t100776;
    let t100784 = t420 * t25759;
    let t100785 = t379 * t3066;
    let t100789 = t3188 * t3066;
    let t100800 = t92557 * t6449;
    (t100753, t100763, t100777, t100784, t100785, t100789, t100800)
}
