//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1246/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1246<F: Float>(t38183: F, t38666: F, t41775: F, t41776: F, t43688: F, t43690: F, t43692: F, t43695: F, t43697: F, t43700: F, t43702: F, t43705: F) -> F {
    let t44510 = -F::cast_from(0.10975748638225852664e0_f64) * t43688 + F::cast_from(0.17336443480108537126e0_f64) * t43690 + F::cast_from(0.5854464323629669811e-1_f64) * t43692 - F::cast_from(0.32927245914677557993e-1_f64) * t38183 + t38666 + t41775 - F::cast_from(0.25610080155860322883e0_f64) * t43695 - F::cast_from(0.86682217400542685632e-1_f64) * t43697 - F::cast_from(0.86682217400542685632e-1_f64) * t43700 - F::cast_from(0.86682217400542685632e-1_f64) * t43702 - t41776 + F::cast_from(0.13099107994629972538e-1_f64) * t43705;
    t44510
}
