//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1201/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1201<F: Float>(t1179: F, t1188: F, t12547: F, t1196: F, t300: F, t3488: F, t1198: F, t3531: F, t3539: F, t3543: F, t3535: F, t12485: F, t12487: F, t3523: F) -> (F, F, F, F, F, F, F, F) {
    let t12564 = t1179 * t12547 * t1188;
    let t12566 = F::cast_from(0.5848223622634646207e0_f64) * t1196 * t12564;
    let t12571 = t300 * t3488;
    let t12573 = F::cast_from(0.17544670867903938621e1_f64) * t12571 * t1198;
    let t12575 = F::cast_from(0.17544670867903938621e1_f64) * t3531 * t3539;
    let t12577 = F::cast_from(0.51947577317044391276e2_f64) * t3531 * t3543;
    let t12579 = F::cast_from(0.35089341735807877242e1_f64) * t3531 * t3535;
    let t12581 = t12485 * t12487 * t3523;
    (t12564, t12566, t12571, t12573, t12575, t12577, t12579, t12581)
}
