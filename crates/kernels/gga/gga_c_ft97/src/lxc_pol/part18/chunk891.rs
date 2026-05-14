//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 891/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk891<F: Float>(t23671: F, t379: F, t5891: F, t23657: F, t138: F, t22708: F, t22711: F, t2035: F, t554: F, t5790: F, t1701: F, t2071: F, t5546: F, t22652: F, t538: F, t22511: F, t5819: F) -> (F, F, F, F, F, F, F, F) {
    let t23673 = t23671 * t5891 * t379;
    let t23674 = t23657 * t23673;
    let t23676 = t138 * t22708;
    let t23677 = t23676 * t22711;
    let t23683 = t2035 * t5790 * t554;
    let t23687 = t1701 * t5546 * t2071;
    let t23691 = t1701 * t22652 * t538;
    let t23700 = t5819 * t22511;
    (t23673, t23674, t23676, t23677, t23683, t23687, t23691, t23700)
}
