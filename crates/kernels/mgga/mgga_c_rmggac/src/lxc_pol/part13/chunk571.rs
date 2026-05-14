//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 571/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk571<F: Float>(t5259: F, t7205: F, t3814: F, t7710: F, t5245: F, t645: F, t739: F, t7855: F, t236: F, t830: F, t507: F, t2007: F, t2191: F, t1260: F, t1986: F, t675: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7867 = t5259 * t7205;
    let t7869 = t3814 * t7710;
    let t7877 = t5245 * t645;
    let t7897 = t739 * t7855;
    let t7900 = t236 * t830;
    let t7901 = t507 * t7900;
    let t7903 = t2191 * t2007;
    let t7905 = t1986 * t1260;
    let t7906 = t675 * t7905;
    (t7867, t7869, t7877, t7897, t7900, t7901, t7903, t7905, t7906)
}
