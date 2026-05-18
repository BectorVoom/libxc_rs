//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 609/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk609<F: Float>(t1909: F, t8425: F, t1843: F, t376: F, t89: F, t7822: F, t7775: F, t7778: F, t7748: F, t7758: F, t7768: F, t7791: F, t7796: F, t7809: F, t7813: F, t7817: F, t7827: F, t7831: F) -> (F, F, F) {
    let t8426 = t1909 * t8425;
    let t8430 = t89 * t376 * t1843;
    let t8437 = F::new(2.0) / F::new(9.0) * t7822;
    let t8443 = F::new(4.0) / F::new(27.0) * t7775;
    let t8444 = t7778 / F::new(9.0);
    let t8445 = F::new(2.0) / F::new(3.0) * t7791 + F::new(2.0) / F::new(9.0) * t7796 - F::new(2.0) / F::new(9.0) * t7809 + t7813 / F::new(3.0) + t7817 / F::new(3.0) - t8437 - F::new(2.0) / F::new(3.0) * t7827 - F::new(2.0) / F::new(3.0) * t7831 - t7748 / F::new(9.0) + F::new(2.0) * t7758 - F::new(10.0) / F::new(81.0) * t7768 - t8443 + t8444;
    (t8426, t8430, t8445)
}
