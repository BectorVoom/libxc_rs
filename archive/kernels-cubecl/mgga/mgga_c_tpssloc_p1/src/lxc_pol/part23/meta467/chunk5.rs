//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1373/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1373<F: Float>(t77174: F, t77189: F, t77204: F, t77218: F, t942: F, t951: F, t959: F, t13520: F, t21253: F, t10661: F, t76644: F, t913: F) -> (F, F, F, F) {
    let t77220 = t77174 + t77189 + t77204 + t77218;
    let t77224 = F::cast_from(0.5848223622634646207e0_f64) * t959 * t942 * t77220 * t951;
    let t77226 = F::cast_from(24.0_f64) * t13520 * t21253;
    let t77229 = F::cast_from(24.0_f64) * t10661 * t76644 * t913;
    (t77220, t77224, t77226, t77229)
}
