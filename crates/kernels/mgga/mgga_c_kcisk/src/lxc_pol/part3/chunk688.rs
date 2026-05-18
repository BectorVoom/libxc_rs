//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 688/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk688<F: Float>(t10568: F, t10641: F, t10579: F, t10582: F, t10590: F, t10598: F, t10644: F, t10647: F, t10661: F, t10665: F, t10667: F, t10672: F, t10675: F, t10678: F) -> F {
    let t10738 = F::new(0.93011851851851851854e0) * t10568;
    let t10739 = F::new(0.36514074074074074075e0) * t10641;
    let t10748 = -F::new(0.33218518518518518518e0) * t10579 + F::new(0.11958666666666666667e1) * t10582 - F::new(0.17938e1) * t10590 - F::new(0.29896666666666666667e0) * t10598 - t10738 - t10739 - F::new(0.82156666666666666668e-1) * t10644 + F::new(0.49293999999999999999e0) * t10647 + F::new(0.3071625e0) * t10661 - F::new(0.76790625e-1) * t10665 + F::new(0.1898925e1) * t10667 + F::new(0.142419375e1) * t10672 - F::new(0.28483875e1) * t10675 + F::new(0.46074375e0) * t10678;
    t10748
}
