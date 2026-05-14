//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1303/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1303<F: Float>(t2697: F, t32605: F, t3368: F, t111091: F, t111094: F, t111097: F, t111099: F, t111101: F, t111103: F, t111105: F, t111109: F, t111113: F, t111116: F, t110995: F, t9382: F, t32647: F, t32658: F) -> (F, F, F) {
    let t111119 = t3368 * t32605 * t2697;
    let t111121 = 0.44229166666666666667e-1 * t111091 - 0.56291666666666666668e-1 * t111094 - 0.56291666666666666668e-1 * t111097 - 0.28145833333333333334e-1 * t111099 + 0.13968375e-1 * t111101 + 0.120625e-1 * t111103 + 0.62500000000000000002e-1 * t111105 - 0.69644166666666666665e-2 * t111109 - 0.41786499999999999999e-1 * t111113 + 0.27857666666666666666e-1 * t111116 + 0.14583333333333333334e0 * t111119;
    let t111123 = t110995 * t9382;
    let t111125 = t32647 * t32658;
    (t111121, t111123, t111125)
}
