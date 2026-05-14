//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1438/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1438<F: Float>(t14962: F, t1597: F, t33807: F, t9535: F, t115075: F, t9536: F, t114774: F, t109664: F, t110078: F, t110079: F, t110081: F, t114978: F, t115358: F, t115423: F, t115569: F, t115774: F, t32436: F, t32439: F, t32468: F, t33837: F, t33937: F, t56066: F, t6204: F, t9539: F) -> (F,) {
    let t115913 = t14962 * t1597;
    let t115926 = t33807 * t9535;
    let t115932 = 0.34722222222222222222e-2 * t9536 * t115075;
    let t115941 = 0.61905925925925925925e-2 * t114774;
    let t115942 = 0.31250000000000000001e-1 * t9536 * t6204 * t115913 * t56066 - 0.52083333333333333333e-2 * t9536 * t114978 + 0.10416666666666666667e-1 * t9536 * t115423 - 0.10416666666666666667e-1 * t32436 * t33837 - 0.40208333333333333334e-2 * t109664 * t33837 - 0.34722222222222222222e-2 * t115926 * t9539 + 0.40208333333333333334e-2 * t32439 * t115569 - t115932 + t110078 + 0.12062500000000000001e-1 * t32439 * t115774 + 0.23280625e-2 * t33937 * t115774 + 0.13402777777777777778e-2 * t115358 * t32468 - 0.20104166666666666667e-2 * t110079 - 0.11607361111111111111e-2 * t110081 - t115941;
    (t115942,)
}
