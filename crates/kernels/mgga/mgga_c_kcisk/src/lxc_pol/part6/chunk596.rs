//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 596/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk596<F: Float>(t4443: F, t4450: F, t5668: F, t5736: F, t7738: F, t7742: F, t7746: F, t7758: F, t7765: F, t7771: F, t7773: F, t7777: F, t7780: F, t7783: F) -> F {
    let t8365 = -F::new(0.17648625e1) * t7758 + F::new(0.3529725e1) * t7765 + t4443 + F::new(0.34431666666666666666e0) * t5668 - F::new(0.34431666666666666667e0) * t7738 + F::new(0.103295e1) * t7742 - F::new(0.516475e0) * t7746 + F::new(0.31558125e0) * t7771 + F::new(0.6311625e0) * t7773 + t4450 + F::new(0.13892666666666666667e0) * t5736 - F::new(0.34731666666666666667e-1) * t7777 + F::new(0.20839e0) * t7780 - F::new(0.104195e0) * t7783;
    t8365
}
