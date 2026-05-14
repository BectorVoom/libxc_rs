//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 451/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk451<F: Float>(t1357: F, t1652: F, t5098: F, t570: F, t5126: F, t1737: F, t338: F, t352: F, t1936: F, t321: F, t333: F, t1950: F, t290: F, t1944: F, t171: F, t1811: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5937 = t1357 * t1652;
    let t5942 = t5098 * t570;
    let t5945 = t5126 * t570;
    let t5948 = t338 * t1737;
    let t5949 = t5948 * t352;
    let t5954 = t1936 * t321;
    let t5957 = t1936 * t333;
    let t5960 = t290 * t1950;
    let t5963 = t1944 * t321;
    let t5966 = t1944 * t333;
    let t5969 = t1811 * t171;
    (t5937, t5942, t5945, t5949, t5954, t5957, t5960, t5963, t5966, t5969)
}
