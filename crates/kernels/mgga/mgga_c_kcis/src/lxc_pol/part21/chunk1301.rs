//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1301/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1301<F: Float>(t14430: F, t9985: F, t95547: F, t95915: F, t1130: F, t2178: F, t13411: F, t3200: F, t13416: F, t4554: F, t26685: F, t95781: F) -> (F, F, F, F) {
    let t95921 = t14430 * t9985;
    let t95923 = t95921 * t95915 * t95547;
    let t95926 = t2178 * t1130;
    let t95928 = t3200 * t95926 * t13411;
    let t95931 = t4554 * t95926 * t13416;
    let t95938 = F::cast_from(0.20612155671296296296e-4_f64) * t26685 * t95781;
    (t95923, t95928, t95931, t95938)
}
