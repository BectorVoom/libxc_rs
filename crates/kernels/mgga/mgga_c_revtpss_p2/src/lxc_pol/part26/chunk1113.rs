//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1113/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1113<F: Float>(t10298: F, t607: F, t2242: F, t2259: F, t11061: F, t30: F, t27383: F, t50066: F, t25207: F, t51775: F, t41161: F, t51792: F) -> (F, F, F, F, F, F, F) {
    let t92709 = t10298 * t607;
    let t92711 = t2242 * t2259;
    let t92743 = t30 * t11061;
    let t92747 = t27383 * t50066;
    let t92753 = t25207 * t51775;
    let t92759 = t25207 * t41161;
    let t92762 = t27383 * t51792;
    (t92709, t92711, t92743, t92747, t92753, t92759, t92762)
}
