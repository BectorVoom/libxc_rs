//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1005/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1005<F: Float>(t10029: F, t7416: F, t2464: F, t2465: F, t2684: F, t7258: F, t22424: F, t3311: F, t161: F, t165: F, t7112: F, t2685: F, t10023: F, t22405: F, t7297: F, t900: F) -> (F, F, F, F, F, F, F) {
    let t28822 = t7416 * t10029;
    let t28827 = 0.17041300423964777634e0 * t2684 * t2464 * t2465 * t7258;
    let t28828 = t22424 * t3311;
    let t28831 = t161 * t165 * t7112;
    let t28833 = t2684 * t2685 * t28831;
    let t28836 = 0.89376224879626066674e-1 * t10023 * t22405;
    let t28837 = t900 * t7297;
    (t28822, t28827, t28828, t28831, t28833, t28836, t28837)
}
