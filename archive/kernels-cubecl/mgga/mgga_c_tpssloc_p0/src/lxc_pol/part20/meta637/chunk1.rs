//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2343/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2343<F: Float>(t2986: F, t2990: F, t48046: F, t42771: F, t4514: F, t43057: F, t13913: F, t2960: F, t4542: F, t698: F, t973: F, t10186: F, t13788: F) -> (F, F, F, F, F, F) {
    let t48048 = t2986 * t48046 * t2990;
    let t48052 = t2986 * t42771 * t4514;
    let t48061 = t2986 * t43057 * t4514;
    let t48063 = t2960 * t13913;
    let t48066 = t973 * t698 * t4542;
    let t48067 = F::cast_from(0.55555555555555555554e-3_f64) * t48066;
    let t48068 = t10186 * t13788;
    (t48048, t48052, t48061, t48063, t48067, t48068)
}
