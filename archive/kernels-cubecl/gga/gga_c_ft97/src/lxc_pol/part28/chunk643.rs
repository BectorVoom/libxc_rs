//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 643/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk643<F: Float>(t23323: F, t3183: F, t1882: F, t6549: F, t110: F, t1871: F, t26001: F, t5635: F, t986: F, t26048: F, t83: F, t6535: F, t8392: F) -> (F, F, F, F, F, F) {
    let t26249 = t23323 * t3183;
    let t26252 = t1882 * t6549;
    let t26255 = t1871 * t110 * t26001;
    let t26259 = t1871 * t986 * t5635;
    let t26262 = t83 * t26048;
    let t26265 = t8392 * t6535;
    (t26249, t26252, t26255, t26259, t26262, t26265)
}
