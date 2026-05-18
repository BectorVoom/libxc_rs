//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1226/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1226<F: Float>(t25305: F, t92868: F, t25292: F, t25322: F, t25326: F, t25344: F, t25383: F, t25391: F, t25394: F, t2772: F, t92841: F, t92844: F, t92847: F, t92856: F, t92858: F, t92861: F, t92864: F, t92870: F, t92873: F) -> F {
    let t92875 = F::new(0.91399340044406952588e-2) * t25305 * t92868;
    let t92876 = -F::new(0.15421710918628844643e0) * t92841 + F::new(0.86736281882051994623e-1) * t92844 + F::new(0.29272321618148349057e-1) * t92847 + F::new(0.26020884564615598386e1) * t25383 * t25344 + F::new(0.26020884564615598386e1) * t25383 * t25326 + F::new(0.52041769129231196772e1) * t25383 * t25292 + F::new(0.16463622957338778996e-1) * t92856 - F::new(0.21951497276451705329e-1) * t92858 + t92861 + F::new(0.39512695097613069591e1) * t25322 * t2772 - F::new(0.52041769129231196772e1) * t25391 * t92864 * t25394 - t92870 - t92873 + t92875;
    t92876
}
