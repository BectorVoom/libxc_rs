//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1005/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1005<F: Float>(t22672: F, t2684: F, t3295: F, t10017: F, t2464: F, t2615: F, t10014: F, t7416: F, t1423: F, t2563: F, t10023: F, t2021: F, t7339: F, t7372: F, t3296: F, t6100: F) -> (F, F, F, F, F, F, F) {
    let t28876 = 0.11928910296775344344e1 * t2684 * t22672 * t3295;
    let t28878 = t2615 * t2464 * t10017;
    let t28880 = t7416 * t10014;
    let t28889 = t1423 * t2563;
    let t28891 = 0.3575048995185042667e0 * t10023 * t28889;
    let t28915 = 0.59584149919750711116e-1 * t2021 * t7339 * t7372;
    let t28916 = t6100 * t3296;
    (t28876, t28878, t28880, t28889, t28891, t28915, t28916)
}
