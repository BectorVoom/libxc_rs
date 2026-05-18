//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 304/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk304<F: Float>(t1695: F, t45: F, t625: F, t630: F, t1718: F, t633: F) -> (F, F, F, F, F, F, F) {
    let t1729 = F::new(0.92708333333333333333e-2) * t1695;
    let t1735 = t45 * t625;
    let t1736 = t630 * t630;
    let t1737 = F::new(1.0) / t1736;
    let t1739 = F::new(0.301925e0) * t1695;
    let t1742 = F::new(0.16557e0) * t1718;
    let t1746 = F::new(1.0) / t633;
    (t1729, t1735, t1736, t1737, t1739, t1742, t1746)
}
