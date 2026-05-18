//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 224/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk224<F: Float>(t322: F, t333: F, t321: F, t338: F, t352: F, t189: F, t280: F, t816: F) -> (F, F, F, F, F) {
    let t904 = t322 * t333;
    let t908 = t338 * t321;
    let t909 = t908 * t352;
    let t912 = F::new(1.0) / t189;
    let t913 = t280 * t280;
    let t916 = F::new(2.0) * t816;
    (t904, t909, t912, t913, t916)
}
