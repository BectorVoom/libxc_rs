//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 666/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk666<F: Float>(t4733: F, t4736: F, t4739: F, t4849: F, t4850: F, t4851: F, t4852: F, t4853: F, t453: F, t1379: F, t445: F, t76: F) -> (F, F, F, F) {
    let t4854 = -F::cast_from(0.34523333333333333333e1_f64) * t4733 + F::cast_from(0.23015555555555555556e1_f64) * t4736 - F::cast_from(0.26851481481481481482e1_f64) * t4739 - t4849 + t4850 - t4851 - t4852 - t4853;
    let t4855 = t4854 * t453;
    let t4859 = F::new(1.0) / t1379 / t445;
    let t4860 = t76 * t4859;
    (t4854, t4855, t4859, t4860)
}
