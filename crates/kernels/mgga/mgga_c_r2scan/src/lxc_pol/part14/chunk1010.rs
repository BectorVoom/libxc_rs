//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1010/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1010<F: Float>(t11725: F, t11728: F, t11730: F, t11732: F, t11734: F, t11737: F, t11739: F, t11742: F, t11745: F, t11749: F, t11751: F, t12158: F) -> F {
    let t12159 = -F::new(0.43663693315433241794e-2) * t11725 + F::new(0.69345773920434148507e0) * t11728 + F::new(0.25610080155860322883e0) * t11730 - F::new(0.10975748638225852664e0) * t11732 - F::new(0.86682217400542685632e-1) * t11734 - F::new(0.86682217400542685632e-1) * t11737 - F::new(0.2600466522016280569e0) * t11739 - F::new(0.2600466522016280569e0) * t11742 - F::new(0.86682217400542685632e-1) * t11745 - F::new(0.2600466522016280569e0) * t11749 + F::new(0.10975748638225852664e0) * t11751 - t12158;
    t12159
}
