//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 985/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk985<F: Float>(t374: F, t983: F, t5078: F, t3463: F, t5048: F, t1196: F, t5169: F, t1195: F, t5067: F, t1187: F, t10752: F, t380: F) -> (F, F, F, F, F, F) {
    let t14857 = t374 * t983;
    let t14858 = t14857 * t5078;
    let t14860 = t3463 * t983;
    let t14861 = t14860 * t5048;
    let t14863 = t5169 * t1196;
    let t14865 = t1195 * t5067;
    let t14866 = t1187 * t14865;
    let t14868 = t380 * t10752;
    (t14858, t14861, t14863, t14865, t14866, t14868)
}
