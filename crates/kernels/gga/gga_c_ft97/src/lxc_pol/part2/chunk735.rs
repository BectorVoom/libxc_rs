//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 735/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk735<F: Float>(t1882: F, t3235: F, t8232: F, t981: F, t110: F, t8326: F, t10974: F, t1780: F, t488: F, t1911: F, t2983: F, t1876: F, t3238: F, t452: F) -> (F, F, F, F, F) {
    let t11549 = F::new(2.0) / F::new(9.0) * t1882 * t3235;
    let t11550 = t8232 * t981;
    let t11552 = t8326 * t110;
    let t11553 = t11552 * t10974;
    let t11556 = t1780 * t488;
    let t11557 = t2983 * t1911;
    let t11558 = t11556 * t11557;
    let t11562 = t452 * t3238 * t1876;
    (t11549, t11550, t11553, t11558, t11562)
}
