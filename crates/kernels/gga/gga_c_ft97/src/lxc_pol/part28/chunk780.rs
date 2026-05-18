//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 780/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk780<F: Float>(t32415: F, t83: F, t23323: F, t5718: F, t103: F, t7211: F, t379: F, t1902: F, t23339: F, t5722: F, t11810: F, t26166: F, t5731: F) -> (F, F, F, F, F, F, F, F) {
    let t32591 = t83 * t32415;
    let t32594 = t23323 * t5718;
    let t32597 = t103 * t7211;
    let t32598 = t32597 * t379;
    let t32599 = t1902 * t32598;
    let t32602 = t23339 * t5722;
    let t32603 = t11810 * t32602;
    let t32606 = t26166 * t5731;
    (t32591, t32594, t32597, t32598, t32599, t32602, t32603, t32606)
}
