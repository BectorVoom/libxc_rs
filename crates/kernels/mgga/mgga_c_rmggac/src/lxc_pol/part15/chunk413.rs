//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 413/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk413<F: Float>(t31: F, t3899: F, t27: F, t32: F, t124: F, t128: F, t325: F, t899: F) -> (F, F, F, F, F) {
    let t3900 = t31 * t3899;
    let t3901 = F::new(308.0) / F::new(27.0) * t3900;
    let t3907 = t27 * t32 * t3899;
    let t3908 = F::new(0.57037037037037037036e0) * t3907;
    let t3924 = t124 * t128;
    let t3928 = t899 * t325;
    (t3900, t3901, t3908, t3924, t3928)
}
