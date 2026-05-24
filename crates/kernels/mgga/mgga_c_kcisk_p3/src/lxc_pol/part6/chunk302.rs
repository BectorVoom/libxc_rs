//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 302/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk302<F: Float>(t617: F, t608: F, t609: F, t1695: F, t606: F, t164: F, t353: F, t579: F, t24: F, t657: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1704 = t617 * t617;
    let t1705 = F::new(1.0) / t1704;
    let t1706 = t608 * t1705;
    let t1707 = F::new(1.0) / t609;
    let t1712 = F::cast_from(0.29896666666666666667e0_f64) * t1695;
    let t1714 = F::sqrt(t606);
    let t1718 = t353 * t164 * t579;
    let t1719 = F::cast_from(0.16431333333333333333e0_f64) * t1718;
    let t1720 = t24 * t657;
    (t1704, t1705, t1706, t1707, t1712, t1714, t1718, t1719, t1720)
}
