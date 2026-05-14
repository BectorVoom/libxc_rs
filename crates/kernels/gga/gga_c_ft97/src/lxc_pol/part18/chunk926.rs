//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 926/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk926<F: Float>(t2223: F, t24081: F, t24080: F, t376: F, t5844: F, t1349: F, t5848: F, t1389: F, t149: F, t1953: F, t23403: F, t23406: F, t23410: F, t23413: F, t23542: F, t24057: F, t24061: F, t24066: F, t24070: F, t24074: F, t24078: F, t564: F, t5766: F, t5772: F, t5775: F, t5781: F, t5845: F, t5849: F, t5973: F) -> (F, F, F, F, F, F, F) {
    let t24082 = t24081 * t2223;
    let t24083 = t24080 * t24082;
    let t24087 = t376 * t5844;
    let t24088 = t1349 * t24087;
    let t24094 = t376 * t5848;
    let t24095 = t1349 * t24094;
    let t24100 = t1349 * t23403 + t23406 / 27.0 - t5772 * t23410 / 9.0 - t23413 * t5775 / 9.0 - t149 * t24057 - 2.0 / 3.0 * t1349 * t24061 - t1349 * t24066 / 3.0 - 2.0 / 3.0 * t1349 * t24070 + 2.0 / 9.0 * t24074 - 2.0 / 3.0 * t5766 * t5781 + 2.0 * t24078 + 2.0 / 9.0 * t5772 * t24083 + 8.0 * t23542 - t24088 / 9.0 + t5766 * t5849 / 3.0 + t5766 * t5845 / 3.0 - t24095 / 9.0 - 2.0 * t564 * t5973 - t1953 * t1389;
    (t24082, t24083, t24087, t24088, t24094, t24095, t24100)
}
