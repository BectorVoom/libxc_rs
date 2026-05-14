//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 551/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk551<F: Float>(t1746: F, t4928: F, t4929: F, t4834: F, t4887: F, t4838: F, t4842: F, t4845: F, t4848: F, t4866: F, t4874: F, t4882: F, t4884: F, t4891: F, t4895: F, t4898: F, t4901: F) -> (F, F) {
    let t4931 = t4928 * t4929 * t1746;
    let t4936 = 0.40256666666666666667e0 * t4834;
    let t4943 = 0.27595e0 * t4887;
    let t4948 = -0.1294625e1 * t4866 + 0.258925e1 * t4874 + t4936 + 0.20128333333333333334e0 * t4838 - 0.20128333333333333333e0 * t4842 + 0.60385e0 * t4845 - 0.301925e0 * t4848 + 0.82524375e-1 * t4882 + 0.16504875e0 * t4884 + t4943 + 0.22076e0 * t4891 - 0.5519e-1 * t4895 + 0.33114e0 * t4898 - 0.16557e0 * t4901;
    (t4931, t4948)
}
