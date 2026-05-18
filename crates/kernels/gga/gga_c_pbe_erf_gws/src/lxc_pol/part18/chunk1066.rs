//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1066/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1066<F: Float>(t6617: F, t2142: F, t3805: F, t2323: F, t3871: F, t9144: F, t3131: F, t3139: F, t3166: F, t2168: F, t3912: F, t6335: F) -> (F, F, F, F, F, F, F) {
    let t11974 = F::new(35.0) / F::new(432.0) * t6617;
    let t11975 = t3805 * t2142;
    let t11976 = F::new(7.0) / F::new(288.0) * t11975;
    let t11977 = t2323 * t3871;
    let t11979 = F::new(35.0) / F::new(216.0) * t9144;
    let t11981 = t3139 * t3131 * t3166;
    let t11983 = t2168 * t11981 / F::new(48.0);
    let t11984 = t3912 * t6335;
    (t11974, t11976, t11977, t11979, t11981, t11983, t11984)
}
