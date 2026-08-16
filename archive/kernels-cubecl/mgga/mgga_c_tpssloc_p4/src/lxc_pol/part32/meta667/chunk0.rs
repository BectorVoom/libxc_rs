//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2099/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2099<F: Float>(t24847: F, t64825: F, t974: F, t8067: F, t85660: F, t11715: F, t491: F, t85964: F, t27488: F, t8070: F, t225: F, t27654: F) -> (F, F, F, F, F, F) {
    let t94963 = t24847 * t974 * t64825;
    let t94966 = t85660 * t8067;
    let t95000 = t85964 * t11715 * t491;
    let t95005 = t85964 * t27488;
    let t95033 = t85660 * t8070;
    let t95035 = t27654 * t225;
    (t94963, t94966, t95000, t95005, t95033, t95035)
}
