//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1249/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1249<F: Float>(t20883: F, t4170: F, t4160: F, t20882: F, t5662: F, t5661: F, t4142: F, t7030: F, t11913: F, t7101: F, t3728: F, t7207: F) -> (F, F, F, F, F) {
    let t20884 = t4170 * t20883;
    let t20885 = t4160 * t20884;
    let t20887 = t5662 * t20882;
    let t20888 = t4170 * t20887;
    let t20889 = t5661 * t20888;
    let t20892 = t4142 * t7030;
    let t20894 = t11913 * t7101;
    let t20898 = t3728 * t7207;
    (t20885, t20889, t20892, t20894, t20898)
}
