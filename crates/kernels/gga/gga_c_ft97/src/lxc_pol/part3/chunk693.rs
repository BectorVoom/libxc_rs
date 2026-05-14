//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 693/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk693<F: Float>(t12020: F, t16150: F, t3193: F, t432: F, t4417: F, t3187: F, t1902: F, t492: F, t8424: F, t1909: F, t3194: F, t18: F, t920: F, t4612: F, t8506: F, t3255: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16151 = t12020 * t16150;
    let t16152 = t3193 * t16151;
    let t16155 = t4417 * t432;
    let t16156 = t3187 * t16155;
    let t16157 = t1902 * t16156;
    let t16160 = t4417 * t492;
    let t16161 = t8424 * t16160;
    let t16162 = t1909 * t16161;
    let t16165 = t3194 * t16150;
    let t16166 = t1909 * t16165;
    let t16169 = t920 * t18;
    let t16170 = t3187 * t16169;
    let t16171 = t1909 * t16170;
    let t16174 = t8506 * t4612;
    let t16177 = t920 * t3255;
    (t16152, t16155, t16157, t16160, t16162, t16166, t16169, t16171, t16174, t16177)
}
