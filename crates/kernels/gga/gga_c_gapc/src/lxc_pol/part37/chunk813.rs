//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 813/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk813<F: Float>(t10273: F, t799: F, t3250: F, t828: F, t2209: F, t3255: F, t6188: F, t772: F, t3243: F, t10247: F, t10250: F, t10253: F, t10258: F, t10262: F, t10267: F, t10271: F) -> (F, F, F, F, F) {
    let t10274 = t799 * t10273;
    let t10276 = t828 * t3250;
    let t10278 = t2209 * t3255;
    let t10280 = t772 * t6188;
    let t10281 = t3243 * t10280;
    let t10283 = 0.69596735221749395468e-7 * t10247 - 0.2087902056652481864e-5 * t10250 - 0.11742981196020707897e-5 * t10253 - 0.74922666485027954031e-6 * t10258 - 0.12374299522427042515e-6 * t10262 + 0.2087902056652481864e-5 * t10267 - 0.11742981196020707897e-4 * t10271 - 0.33406432906439709826e-4 * t10274 + 0.74372214241464483348e-4 * t10276 + 0.23404413911513494211e-4 * t10278 - 0.11742981196020707897e-5 * t10281;
    (t10274, t10276, t10278, t10281, t10283)
}
