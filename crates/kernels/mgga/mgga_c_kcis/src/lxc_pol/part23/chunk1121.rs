//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1121/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1121<F: Float>(t1464: F, t15955: F, t27387: F, t3722: F, t16974: F, t2237: F, t27369: F, t27372: F, t28369: F, t28535: F, t6176: F, t7895: F, t7914: F, t94539: F, t94546: F, t94554: F, t98119: F, t98445: F, t98538: F, t98543: F) -> (F, F) {
    let t98553 = t1464 * t27387 * t15955 * t3722;
    let t98561 = -0.73697530864197530861e-3 * t94539 + 0.61836467013888888888e-4 * t98538 - 0.46336805555555555556e-3 * t94546 + 0.23168402777777777778e-3 * t94554 - 0.55273148148148148147e-3 * t98543 + 0.13901041666666666667e-2 * t7895 * t28535 + 0.69505208333333333333e-3 * t2237 * t6176 * t7914 * t16974 - 0.16581944444444444444e-2 * t98553 - 0.92754700520833333333e-4 * t27369 * t98445 - 0.13901041666666666667e-2 * t28369 * t27372 - 0.18550940104166666667e-3 * t98119 * t27372;
    (t98553, t98561)
}
