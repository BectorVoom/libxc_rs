//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 924/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk924<F: Float>(t3554: F, t7411: F, t10867: F, t1901: F, t5776: F, t10769: F, t10801: F, t10803: F, t10807: F, t10812: F, t10814: F, t10816: F, t10823: F, t10827: F, t5783: F, t5790: F, t7332: F, t7357: F, t9148: F, t9185: F, t9192: F) -> (F, F, F, F) {
    let t10900 = 0.48245938496077605201e2 * t7411 * t3554;
    let t10901 = t10867 * t1901;
    let t10903 = 0.96491876992155210402e2 * t5776 * t10901;
    let t10918 = 0.142419375e1 * t10801 - 0.28483875e1 * t10803 + 0.1898925e1 * t10807 - t5783 + 0.11958666666666666667e1 * t7357 - 0.89690000000000000001e0 * t9148 + 0.8969e0 * t10769 - 0.76790625e-1 * t10812 + 0.46074375e0 * t10814 + 0.3071625e0 * t10816 - t5790 + 0.82156666666666666666e0 * t7332 - 0.49293999999999999999e0 * t9185 - 0.49293999999999999999e0 * t9192 + 0.73941e0 * t10823 + 0.24647e0 * t10827;
    (t10900, t10901, t10903, t10918)
}
