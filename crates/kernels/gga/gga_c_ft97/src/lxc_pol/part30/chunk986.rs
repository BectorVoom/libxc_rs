//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 986/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk986<F: Float>(t35778: F, t92: F, t51892: F, t7546: F, t24237: F, t35259: F, t1168: F, t2568: F, t33489: F, t35251: F, t28023: F, t6187: F) -> (F, F, F, F, F, F) {
    let t149743 = t35778 * t92;
    let t149748 = t51892 * t7546;
    let t149750 = t24237 * t35259;
    let t149753 = t2568 * t33489 * t1168;
    let t149760 = t24237 * t35251;
    let t149764 = t28023 * t6187;
    (t149743, t149748, t149750, t149753, t149760, t149764)
}
