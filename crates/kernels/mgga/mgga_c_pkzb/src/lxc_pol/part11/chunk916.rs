//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 916/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk916<F: Float>(t6177: F, t6256: F, t7950: F, t8059: F, t8060: F, t9812: F, t9814: F, t9819: F, t9823: F, t9826: F, t9830: F, t9834: F) -> F {
    let t9957 = F::new(0.31558125e0) * t9812 + F::new(0.6311625e0) * t9814 - t6256 + F::new(0.34731666666666666666e0) * t6177 + F::new(0.69463333333333333333e0) * t7950 - t8059 - t8060 - F::new(0.20839e0) * t9819 + F::new(0.62517e0) * t9823 - F::new(0.20839e0) * t9826 + F::new(0.312585e0) * t9830 + F::new(0.312585e0) * t9834;
    t9957
}
