//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 948/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk948<F: Float>(t10727: F, t2792: F, t2836: F, t2844: F, t912: F, t2842: F, t2880: F, t933: F, t10662: F, t913: F, t2860: F, t919: F) -> (F, F, F, F, F) {
    let t10729 = F::cast_from(6.0_f64) * t2792 * t10727;
    let t10731 = t2836 * t2844 * t912;
    let t10733 = F::cast_from(0.48245938496077605201e2_f64) * t2842 * t10731;
    let t10734 = t933 * t2880;
    let t10737 = t10662 * t913;
    let t10739 = F::cast_from(6.0_f64) * t2842 * t10737;
    let t10740 = t919 * t2860;
    (t10729, t10733, t10734, t10739, t10740)
}
