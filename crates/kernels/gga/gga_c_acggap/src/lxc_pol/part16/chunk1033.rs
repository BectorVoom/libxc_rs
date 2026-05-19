//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1033/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1033<F: Float>(t36351: F, t31773: F, t8916: F, t7447: F, t8920: F, t1439: F, t1983: F, t7380: F, t1460: F, t1992: F, t2095: F, t30225: F, t532: F) -> (F, F, F, F, F, F) {
    let t36352 = F::cast_from(0.12862205435420921092e-2_f64) * t36351;
    let t36353 = t31773 * t8916;
    let t36354 = F::new(0.3361875e0) * t36353;
    let t36355 = t7447 * t8920;
    let t36356 = F::new(0.16809375e0) * t36355;
    let t36364 = t7380 * t1983 * t1439;
    let t36365 = t36364 / F::new(32.0);
    let t36367 = t2095 * t1992 * t1460;
    let t36368 = t36367 / F::new(48.0);
    let t36370 = t30225 * t532;
    (t36352, t36354, t36356, t36365, t36368, t36370)
}
