//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1109/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1109<F: Float>(t161: F, t165: F, t7112: F, t2684: F, t2685: F, t10023: F, t22405: F, t7297: F, t900: F, t22295: F, t10040: F, t22883: F) -> (F, F, F, F, F, F) {
    let t28831 = t161 * t165 * t7112;
    let t28833 = t2684 * t2685 * t28831;
    let t28836 = F::cast_from(0.89376224879626066674e-1_f64) * t10023 * t22405;
    let t28837 = t900 * t7297;
    let t28839 = F::cast_from(0.3575048995185042667e0_f64) * t22295 * t28837;
    let t28841 = F::cast_from(0.59584149919750711116e-1_f64) * t22883 * t10040;
    (t28831, t28833, t28836, t28837, t28839, t28841)
}
