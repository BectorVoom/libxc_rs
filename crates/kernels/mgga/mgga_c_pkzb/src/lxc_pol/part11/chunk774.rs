//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 774/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk774<F: Float>(t2575: F, t51: F, t1727: F, t2642: F, t2607: F, t501: F, t2605: F, t496: F, t5086: F, t5143: F, t2557: F, t545: F) -> (F, F, F, F, F, F, F, F) {
    let t6999 = t51 * t2575;
    let t7009 = F::new(0.20007875121765877254e-2) * t1727 * t2642;
    let t7012 = t501 * t2607;
    let t7015 = F::new(8.0) * t496 * t2605;
    let t7017 = F::new(8.0) * t501 * t2605;
    let t7019 = F::new(32.0) * t5086;
    let t7022 = F::new(48.0) * t5143;
    let t7028 = t2557 * t545;
    (t6999, t7009, t7012, t7015, t7017, t7019, t7022, t7028)
}
