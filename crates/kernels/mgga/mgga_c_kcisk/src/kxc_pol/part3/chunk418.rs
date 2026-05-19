//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 418/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk418<F: Float>(t3088: F, t944: F, t151: F, t3107: F, t852: F, t180: F, t182: F, t183: F, t2925: F, t3144: F, t3148: F, t3155: F, t3156: F, t60: F, t983: F, t990: F, t991: F, t995: F) -> (F, F, F) {
    let t3162 = t944 * t3088;
    let t3166 = t151 * t3107;
    let t3170 = t852 * t852;
    let t3174 = -F::cast_from(0.43802864444444444443e-3_f64) * t180 * t3144 * t183 - F::new(0.2e-22) * t990 * t3148 * t183 - F::cast_from(0.26281718666666666666e-2_f64) * t180 * t983 * t995 + F::cast_from(0.19711288999999999999e-2_f64) * t3155 * t3156 + F::cast_from(0.19711288999999999999e-2_f64) * t990 * t991 * t995 + F::cast_from(0.39422577999999999998e-2_f64) * t180 * t182 * t3162 - F::cast_from(0.19711288999999999999e-2_f64) * t180 * t182 * t3166 - F::new(4.0) * t3170 - F::new(4.0) * t60 * t2925;
    (t3162, t3166, t3174)
}
