//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 913/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk913<F: Float>(t6177: F, t6218: F, t7950: F, t8090: F, t8091: F, t9812: F, t9814: F, t9819: F, t9823: F, t9826: F, t9830: F, t9834: F) -> F {
    let t9928 = F::new(0.82524375e-1) * t9812 + F::new(0.16504875e0) * t9814 - t6218 + F::new(0.27595e0) * t6177 + F::new(0.5519e0) * t7950 - t8090 - t8091 - F::new(0.16557e0) * t9819 + F::new(0.49671e0) * t9823 - F::new(0.16557e0) * t9826 + F::new(0.248355e0) * t9830 + F::new(0.248355e0) * t9834;
    t9928
}
