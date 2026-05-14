//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1427/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1427<F: Float>(t109514: F, t33770: F, t32439: F, t109664: F, t109828: F, t109832: F, t109836: F, t109838: F, t109846: F, t114368: F, t114371: F, t115085: F, t115099: F, t115150: F, t115304: F, t32436: F, t32468: F, t33771: F, t33817: F, t9536: F) -> (F, F) {
    let t115645 = t109514 * t33770;
    let t115646 = t32439 * t115645;
    let t115656 = -0.11574074074074074074e-2 * t109828 + 0.20635308641975308642e-2 * t109832 + 0.77382407407407407407e-3 * t109836 - 0.23214722222222222222e-2 * t109838 - 0.77382407407407407406e-3 * t109846 + 0.13402777777777777778e-2 * t115085 * t32468 + 0.13402777777777777778e-2 * t109664 * t33771 + 0.34722222222222222222e-2 * t32436 * t33817 + 0.44675925925925925926e-3 * t115646 - 0.34722222222222222222e-2 * t9536 * t115150 - 0.25794135802469135802e-3 * t114368 - 0.17411041666666666666e-2 * t114371 - 0.10416666666666666667e-1 * t9536 * t115099 - 0.20833333333333333334e-1 * t9536 * t115304;
    (t115645, t115656)
}
