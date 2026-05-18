//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1023/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1023<F: Float>(t137: F, t136: F, t627: F, t6896: F, t130: F, t131: F, t142: F, t20816: F, t2003: F, t2010: F, t623: F, t6944: F) -> (F, F, F, F, F) {
    let t22834 = t137 * t137;
    let t22835 = F::new(1.0) / t22834;
    let t22836 = t136 * t22835;
    let t22850 = t6896 * t627;
    let t22856 = F::new(0.36717874996221960261e1) * t130 * t131 * t20816 * t142;
    let t22889 = t2003 * t2010;
    let t22892 = t623 * t6944;
    (t22836, t22850, t22856, t22889, t22892)
}
