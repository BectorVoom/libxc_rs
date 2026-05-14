//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 900/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk900<F: Float>(t11753: F, t11725: F, t11728: F, t11730: F, t11732: F, t11734: F, t11737: F, t11739: F, t11742: F, t11745: F, t11749: F, t11751: F, t11758: F, t11762: F, t11766: F, t11772: F) -> (F, F, F, F, F) {
    let t12158 = 0.19514881078765566037e-1 * t11753;
    let t12159 = -0.43663693315433241794e-2 * t11725 + 0.69345773920434148507e0 * t11728 + 0.25610080155860322883e0 * t11730 - 0.10975748638225852664e0 * t11732 - 0.86682217400542685632e-1 * t11734 - 0.86682217400542685632e-1 * t11737 - 0.2600466522016280569e0 * t11739 - 0.2600466522016280569e0 * t11742 - 0.86682217400542685632e-1 * t11745 - 0.2600466522016280569e0 * t11749 + 0.10975748638225852664e0 * t11751 - t12158;
    let t12162 = 0.54878743191129263322e-2 * t11758;
    let t12163 = 0.46574606203128791246e-1 * t11762;
    let t12164 = 0.13972381860938637374e0 * t11766;
    let t12166 = 0.46574606203128791246e-1 * t11772;
    (t12159, t12162, t12163, t12164, t12166)
}
