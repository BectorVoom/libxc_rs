//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 800/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk800<F: Float>(t7438: F, t7585: F, t314: F, t7112: F, t313: F, t2154: F, t954: F, t2717: F, t769: F, t836: F, t568: F, t808: F) -> (F, F, F, F, F, F, F) {
    let t7586 = t7585 * t7438;
    let t7589 = t314 * t7112;
    let t7590 = t313 * t7589;
    let t7593 = t2154 * t954;
    let t7596 = t769 * t2717;
    let t7601 = t836 * t7112;
    let t7602 = t568 * t7601;
    let t7607 = t808 * t7112;
    (t7586, t7589, t7590, t7593, t7596, t7602, t7607)
}
