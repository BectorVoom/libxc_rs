//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1247/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1247<F: Float>(t11698: F, t3577: F, t248: F, t3494: F, t3570: F, t1213: F, t3490: F, t3523: F, t1190: F, t3030: F, t3032: F, t3505: F) -> (F, F, F, F, F, F) {
    let t11699 = t3577 * t11698;
    let t11702 = t248 * t3570 * t3494;
    let t11703 = t1213 * t11702;
    let t11705 = t3490 * t3523;
    let t11707 = t1190 * t3030;
    let t11708 = t11707 * t3032;
    let t11709 = t11708 * t3505;
    (t11699, t11703, t11705, t11707, t11708, t11709)
}
