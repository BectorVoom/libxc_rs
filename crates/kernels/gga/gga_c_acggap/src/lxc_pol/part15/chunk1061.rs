//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1061/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1061<F: Float>(t34210: F, t34211: F, t34212: F, t34214: F, t34215: F, t36972: F, t36976: F, t36984: F, t36987: F, t39080: F, t39082: F, t39086: F, t39088: F, t39092: F, t39094: F, t39098: F, t39100: F, t39107: F) -> (F,) {
    let t41479 = -0.17149607247227894789e-2 * t39080 + 0.17149607247227894789e-2 * t39082 - t36972 - t36976 - 0.17149607247227894789e-2 * t39086 + 0.17149607247227894789e-2 * t39088 - 0.14291339372689912324e-3 * t39092 - 0.32012600194825403606e-1 * t39094 + 0.94344276868812456204e-2 * t39098 - 0.18868855373762491241e-2 * t39100 - t34210 - t34211 - t34212 + t34214 - 0.12579236915841660828e-2 * t34215 - t36984 - t36987 - 0.64025200389650807211e-1 * t39107;
    (t41479,)
}
