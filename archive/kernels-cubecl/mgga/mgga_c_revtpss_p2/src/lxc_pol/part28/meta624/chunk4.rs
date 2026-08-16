//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2218/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2218<F: Float>(t4797: F, t7131: F, t1068: F, t15970: F, t27493: F, t4788: F, t93597: F, t93687: F, t93689: F, t93694: F, t93696: F, t93702: F, t93704: F, t93713: F, t93718: F, t93720: F) -> F {
    let t100230 = t4797 * t7131;
    let t100233 = -F::cast_from(0.3811023832717309953e-3_f64) * t93687 + F::cast_from(0.57165357490759649296e-3_f64) * t93689 + F::cast_from(0.57165357490759649296e-3_f64) * t27493 * t15970 - F::cast_from(0.30488190661738479624e-2_f64) * t93597 * t4788 - t93694 / F::cast_from(162.0_f64) - t93696 / F::cast_from(648.0_f64) + t93702 / F::cast_from(864.0_f64) + t93704 / F::cast_from(648.0_f64) + F::cast_from(0.57165357490759649296e-3_f64) * t93713 + F::cast_from(0.30488190661738479624e-2_f64) * t93718 + F::cast_from(0.19055119163586549765e-3_f64) * t93720 + F::cast_from(0.57165357490759649296e-3_f64) * t100230 * t1068;
    t100233
}
