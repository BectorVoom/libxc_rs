//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 558/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk558<F: Float>(t23054: F, t5677: F, t2: F, t5617: F, t1322: F, t1636: F, t89: F, t375: F, t5700: F, t376: F, t5623: F, t23037: F) -> (F, F, F, F, F, F, F) {
    let t23055 = t23054 * t5677;
    let t23057 = t2 * t5617;
    let t23075 = t89 * t1636 * t1322;
    let t23076 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t23075;
    let t23081 = t89 * t375 * t5700;
    let t23089 = t376 * t5623;
    let t23114 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t23037;
    (t23055, t23057, t23075, t23076, t23081, t23089, t23114)
}
