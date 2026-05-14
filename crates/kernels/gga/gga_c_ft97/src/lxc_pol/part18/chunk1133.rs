//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1133/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1133<F: Float>(t1882: F, t23502: F, t23507: F, t23595: F, t23591: F, t23520: F, t23538: F, t23417: F, t8392: F, t38953: F, t5944: F, t2101: F, t5929: F, t23556: F, t23511: F, t23497: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t95634 = t1882 * t23502;
    let t95636 = t1882 * t23507;
    let t95643 = t1882 * t23595;
    let t95649 = t1882 * t23591;
    let t95651 = t1882 * t23520;
    let t95653 = t1882 * t23538;
    let t95659 = t8392 * t23417;
    let t95676 = t38953 * t5944;
    let t95696 = t2101 * t5929;
    let t95707 = t8392 * t23556;
    let t95714 = t8392 * t23511;
    let t95720 = t1882 * t23497;
    (t95634, t95636, t95643, t95649, t95651, t95653, t95659, t95676, t95696, t95707, t95714, t95720)
}
