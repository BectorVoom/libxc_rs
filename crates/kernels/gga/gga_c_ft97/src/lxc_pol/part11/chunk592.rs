//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 592/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk592<F: Float>(t449: F, t8232: F, t1868: F, t1882: F, t110: F, t1866: F, t7959: F, t7748: F, t7758: F, t7768: F, t7775: F, t7778: F, t7791: F, t7796: F, t7809: F, t7813: F, t7817: F, t7822: F, t7827: F, t7831: F) -> (F, F, F, F) {
    let t8233 = t8232 * t449;
    let t8235 = t1882 * t1868;
    let t8238 = t1866 * t110 * t7959;
    let t8252 = F::new(2.0) * t7791 + F::new(2.0) / F::new(3.0) * t7796 - F::new(2.0) / F::new(3.0) * t7809 + t7813 + t7817 - F::new(2.0) / F::new(3.0) * t7822 - F::new(2.0) * t7827 - F::new(2.0) * t7831 - t7748 / F::new(3.0) + F::new(6.0) * t7758 - F::new(10.0) / F::new(27.0) * t7768 - F::new(4.0) / F::new(9.0) * t7775 + t7778 / F::new(3.0);
    (t8233, t8235, t8238, t8252)
}
