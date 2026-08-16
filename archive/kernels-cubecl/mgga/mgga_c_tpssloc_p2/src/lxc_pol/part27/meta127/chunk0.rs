//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 750/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk750<F: Float>(t2764: F, t273: F, t2799: F, t2807: F, t901: F, t241: F, t63: F, t281: F, t283: F, t699: F, t909: F) -> (F, F, F, F, F, F, F, F) {
    let t2810 = F::cast_from(0.39862222222222222223e0_f64) * t2764;
    let t2815 = F::cast_from(1.0_f64)/F::sqrt(t273);
    let t2816 = t2815 * t2799;
    let t2818 = t901 * t2807;
    let t2820 = t63 * t241;
    let t2822 = t281 * t2820 * t283;
    let t2823 = F::cast_from(0.13692777777777777778e0_f64) * t2822;
    let t2824 = t699 * t909;
    (t2810, t2815, t2816, t2818, t2820, t2822, t2823, t2824)
}
