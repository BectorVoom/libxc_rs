//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1353/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1353<F: Float>(t231: F, t2760: F, t2782: F, t2783: F, t836: F, t10871: F, t14545: F, t39709: F, t2645: F, t234: F, t39545: F, t685: F, t875: F) -> (F, F, F, F) {
    let t40278 = t2782 * t2783 * t2760 * t836 * t231;
    let t40282 = t2782 * t14545 * t39709 * t10871;
    let t40284 = t10871 * t2645;
    let t40294 = F::cast_from(0.65457331274007190912e-5_f64) * t39545 * t234 * t875 * t685;
    (t40278, t40282, t40284, t40294)
}
