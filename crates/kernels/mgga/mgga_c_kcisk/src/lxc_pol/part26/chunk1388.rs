//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1388/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1388<F: Float>(t109514: F, t34954: F, t9536: F, t34944: F, t4419: F, t2737: F, t32377: F, t34940: F, t9515: F, t109803: F, t114368: F, t115646: F, t119319: F, t119322: F, t120041: F, t34941: F, t34945: F, t9512: F, t9519: F, t9524: F, t9544: F) -> (F, F, F) {
    let t120490 = t109514 * t34954;
    let t120491 = t9536 * t120490;
    let t120498 = t4419 * t34944;
    let t120499 = t2737 * t120498;
    let t120503 = t32377 * t120498;
    let t120509 = t9515 * t34940;
    let t120517 = 0.11574074074074074074e-2 * t120491 + 0.44675925925925925927e-3 * t115646 - 0.10416666666666666667e-1 * t9512 * t34945 - 0.10416666666666666667e-1 * t9524 * t34945 - 0.34722222222222222223e-2 * t120499 + 0.31040833333333333333e-2 * t109803 * t34945 - 0.38801041666666666667e-3 * t120503 + 0.52083333333333333333e-2 * t34941 * t9544 + 0.52083333333333333333e-2 * t34941 * t9519 + 0.20104166666666666667e-2 * t120509 * t9519 - 0.51588271604938271603e-3 * t114368 - 0.11607361111111111111e-2 * t119319 - 0.51588271604938271603e-3 * t119322 + 0.10416666666666666667e-1 * t9536 * t120041;
    (t120490, t120498, t120517)
}
