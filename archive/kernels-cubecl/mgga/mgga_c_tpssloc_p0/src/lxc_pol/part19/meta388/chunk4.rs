//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1461/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1461<F: Float>(t44792: F, t44793: F, t44795: F, t44796: F, t1174: F, t11765: F, t135: F, t43763: F, t44620: F, t3551: F, t698: F, t11545: F, t43791: F) -> (F, F, F, F, F) {
    let t44798 = t44792 + t44793 + t44795 + t44796;
    let t44803 = t1174 * t135 * t11765;
    let t44805 = t44620 * t43763;
    let t44811 = t1174 * t698 * t3551;
    let t44817 = t11545 * t43791;
    (t44798, t44803, t44805, t44811, t44817)
}
