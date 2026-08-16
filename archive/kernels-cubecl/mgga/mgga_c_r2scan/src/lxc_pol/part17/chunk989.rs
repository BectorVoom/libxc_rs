//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 989/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk989<F: Float>(t11725: F, t11728: F, t11730: F, t11732: F, t11734: F, t11737: F, t11739: F, t11742: F, t11745: F, t11749: F, t11751: F, t12158: F) -> F {
    let t12159 = -F::cast_from(0.43663693315433241794e-2_f64) * t11725 + F::cast_from(0.69345773920434148507e0_f64) * t11728 + F::cast_from(0.25610080155860322883e0_f64) * t11730 - F::cast_from(0.10975748638225852664e0_f64) * t11732 - F::cast_from(0.86682217400542685632e-1_f64) * t11734 - F::cast_from(0.86682217400542685632e-1_f64) * t11737 - F::cast_from(0.2600466522016280569e0_f64) * t11739 - F::cast_from(0.2600466522016280569e0_f64) * t11742 - F::cast_from(0.86682217400542685632e-1_f64) * t11745 - F::cast_from(0.2600466522016280569e0_f64) * t11749 + F::cast_from(0.10975748638225852664e0_f64) * t11751 - t12158;
    t12159
}
