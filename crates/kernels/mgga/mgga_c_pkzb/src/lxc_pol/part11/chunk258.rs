//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 258/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk258<F: Float>(t819: F, t845: F, t826: F, t837: F, t842: F, t849: F) -> (F, F, F) {
    let t865 = F::new(0.516475e0) * t819;
    let t868 = F::new(0.104195e0) * t845;
    let t870 = F::new(0.3529725e1) * t837 - t865 + F::new(0.1549425e1) * t826 + F::new(0.6311625e0) * t842 - t868 + F::new(0.312585e0) * t849;
    (t865, t868, t870)
}
