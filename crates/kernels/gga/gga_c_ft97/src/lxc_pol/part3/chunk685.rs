//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 685/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk685<F: Float>(t3291: F, t452: F, t942: F, t11863: F, t15959: F, t4431: F, t492: F, t1910: F, t1909: F, t363: F, t3187: F, t3194: F, t3193: F, t11902: F, t3205: F, t11430: F, t11436: F, t11448: F, t15978: F, t15980: F, t15983: F, t15987: F, t15991: F, t15996: F, t1901: F, t3281: F, t446: F) -> (F, F) {
    let t16000 = t452 * t3291 * t942;
    let t16003 = t11863 * t15959;
    let t16006 = t4431 * t492;
    let t16007 = t1910 * t16006;
    let t16008 = t1909 * t16007;
    let t16011 = t4431 * t363;
    let t16012 = t3187 * t16011;
    let t16013 = t1909 * t16012;
    let t16016 = t3194 * t16011;
    let t16017 = t3193 * t16016;
    let t16020 = t11902 * t3205;
    let t16023 = t15978 / 9.0 + 2.0 / 9.0 * t15980 + t11430 - t11436 - t11448 - 2.0 / 9.0 * t446 * t15983 - 4.0 / 9.0 * t3281 * t15987 - t446 * t15991 / 9.0 - 2.0 / 3.0 * t446 * t15996 - 2.0 / 3.0 * t446 * t16000 - 4.0 / 9.0 * t1901 * t16003 + t1901 * t16008 / 9.0 + 2.0 / 9.0 * t1901 * t16013 - 2.0 / 27.0 * t1901 * t16017 + 2.0 / 9.0 * t1901 * t16020;
    (t16011, t16023)
}
