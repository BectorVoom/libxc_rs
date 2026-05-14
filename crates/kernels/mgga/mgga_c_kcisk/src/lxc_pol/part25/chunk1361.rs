//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1361/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1361<F: Float>(t33218: F, t964: F, t1310: F, t2021: F, t18775: F, t33198: F, t112761: F, t34500: F, t9740: F, t34495: F, t33196: F, t25: F, t34559: F, t34562: F, t10000: F, t33276: F) -> (F, F, F, F, F, F, F) {
    let t117857 = t964 * t33218;
    let t117866 = t1310 * t2021;
    let t117868 = t117866 * t18775 * t33198;
    let t117873 = 0.23148148148148148148e-2 * t9740 * t112761 * t34500;
    let t117874 = t112761 * t34495;
    let t117876 = 0.44675925925925925926e-3 * t33196 * t117874;
    let t117880 = 0.15432098765432098765e-2 * t9740 * t25 * t34559 * t34562;
    let t117887 = t10000 * t33276;
    (t117857, t117868, t117873, t117874, t117876, t117880, t117887)
}
