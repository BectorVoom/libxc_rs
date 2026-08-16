//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1285/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1285<F: Float>(t321: F, t3431: F, t107: F, t787: F, t2028: F, t3038: F, t7275: F, t1445: F, t2034: F, t2087: F, t28381: F, t32191: F, t33105: F, t33109: F, t33112: F, t33114: F, t33117: F, t33118: F, t33126: F, t33127: F, t33130: F, t33132: F, t33134: F, t33136: F, t723: F, t833: F) -> (F, F) {
    let t33137 = t321 * t3431;
    let t33139 = t787 * t33137 * t107;
    let t33145 = F::cast_from(0.79445533226334281486e-1_f64) * t787 * t7275 * t3038 * t2028;
    let t33146 = -t33105 + t33109 - t33112 - t33114 + t33117 - F::cast_from(0.13803453343411469884e2_f64) * t2087 * t1445 * t33118 * t723 + F::cast_from(0.23005755572352449806e2_f64) * t833 * t1445 * t32191 - t33126 - t33127 + t28381 - t33130 - t33132 + t33134 + t33136 + F::cast_from(0.23833659967900284446e0_f64) * t33139 * t2034 - t33145;
    (t33137, t33146)
}
