//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 932/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk932<F: Float>(t10024: F, t41454: F, t446: F, t683: F, t7514: F, t505: F, t668: F, t9708: F, t1882: F, t9776: F, t2409: F, t2459: F, t2354: F, t2373: F, t2413: F, t9770: F) -> (F, F, F, F, F, F, F, F) {
    let t41823 = t446 * t10024 * t41454;
    let t41825 = t683 * t7514;
    let t41827 = t9708 * t668 * t505;
    let t41829 = t446 * t41825 * t41827;
    let t41831 = t1882 * t9776;
    let t41833 = t2409 * t2459;
    let t41835 = t446 * t2354 * t41833;
    let t41837 = t2413 * t2373;
    let t41839 = t446 * t9770 * t41837;
    (t41823, t41827, t41829, t41831, t41833, t41835, t41837, t41839)
}
