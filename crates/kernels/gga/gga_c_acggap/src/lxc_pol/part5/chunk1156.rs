//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1156/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1156<F: Float>(t3431: F, t6161: F, t18499: F, t18502: F, t18508: F, t18510: F, t18518: F, t18523: F, t18525: F, t18545: F, t18553: F, t18555: F, t372: F, t4261: F, t4262: F, t5544: F) -> (F,) {
    let t23915 = t3431 * t6161;
    let t23927 = 0.34299214494455789577e-2 * t18499 + 0.17149607247227894789e-2 * t18502 - 0.34299214494455789578e-1 * t18508 - 0.40015750243531754508e-2 * t18510 - 0.80031500487063509015e-2 * t23915 + 0.34299214494455789578e-2 * t18518 + 0.34299214494455789578e-2 * t18523 - 0.51448821741683684367e-1 * t18525 - 35.0 / 54.0 * t18545 - 0.34299214494455789578e-2 * t18553 - 0.32012600194825403606e-1 * t18555 - t4261 * t4262 * t5544 * t372 / 6.0;
    (t23927,)
}
