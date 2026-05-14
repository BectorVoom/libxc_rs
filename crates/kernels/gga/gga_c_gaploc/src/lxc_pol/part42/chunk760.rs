//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 760/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk760<F: Float>(t45529: F, t2631: F, t36515: F, t787: F, t10827: F, t11053: F, t9805: F, t2679: F, t3630: F, t9796: F, t11755: F, t2028: F, t2536: F, t11763: F, t13506: F, t4673: F, t6060: F) -> (F, F, F, F, F, F, F) {
    let t45530 = 0.29792074959875355558e-1 * t45529;
    let t45536 = 0.17875244975925213335e2 * t787 * t36515 * t2631;
    let t45542 = t9805 * t11053 * t10827;
    let t45543 = 0.11502877786176224903e1 * t45542;
    let t45548 = t9796 * t3630 * t2679;
    let t45549 = 0.38342925953920749676e0 * t45548;
    let t45553 = 0.39722766613167140743e-1 * t787 * t2536 * t11755 * t2028;
    let t45557 = 0.39722766613167140743e-1 * t787 * t2536 * t11763 * t2028;
    let t45560 = 0.14300195980740170667e1 * t6060 * t4673 * t13506;
    (t45530, t45536, t45543, t45549, t45553, t45557, t45560)
}
