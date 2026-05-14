//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1187/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1187<F: Float>(t35125: F, t10470: F, t4418: F, t10474: F, t4425: F, t20019: F, t26984: F, t6520: F, t10318: F, t4360: F, t4667: F, t10319: F, t4753: F, t2413: F, t26122: F, t26726: F, t901: F) -> (F, F, F, F, F, F, F, F) {
    let t35126 = 0.36425779656224712192e1 * t35125;
    let t35128 = 0.2556195063594716645e1 * t4418 * t10470;
    let t35130 = 0.1022478025437886658e1 * t4425 * t10474;
    let t35133 = 0.23833659967900284446e0 * t26984 * t20019 * t6520;
    let t35136 = 0.71500979903700853338e0 * t4360 * t10318 * t4667;
    let t35138 = 0.47667319935800568892e0 * t10319 * t4753;
    let t35140 = 0.21450293971110256002e1 * t26122 * t2413;
    let t35141 = t26726 * t901;
    (t35126, t35128, t35130, t35133, t35136, t35138, t35140, t35141)
}
