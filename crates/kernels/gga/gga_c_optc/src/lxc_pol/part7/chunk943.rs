//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 943/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk943<F: Float>(t123: F, t3107: F, t1028: F, t8968: F, t1111: F, t1121: F, t1133: F, t8531: F, t8534: F, t8539: F, t8543: F, t8548: F, t8908: F, t8913: F, t8918: F, t8921: F, t8924: F, t8928: F, t8930: F, t8933: F, t8937: F, t8941: F, t8944: F, t8947: F, t8953: F, t8957: F, t8960: F, t8963: F, t8966: F, t8970: F, t8973: F) -> (F, F, F, F) {
    let t8974 = t3107 * t123;
    let t8975 = t8974 * t1028;
    let t8976 = t8968 * t8975;
    let t8979 = t8531 - t1111 * t8534 / F::cast_from(36.0_f64) + t1111 * t8539 / F::cast_from(48.0_f64) - t8543 / F::cast_from(144.0_f64) + t8548 + F::cast_from(0.35500316489081544176e-1_f64) * t1121 * t8908 + F::cast_from(0.10629507243271336419e5_f64) * t8913 * t8918 - F::cast_from(0.10629507243271336419e5_f64) * t8921 * t8924 + F::cast_from(0.71000632978163088351e-1_f64) * t8928 - t1111 * t8930 / F::cast_from(48.0_f64) + t1111 * t8933 / F::cast_from(72.0_f64) + t1111 * t8937 / F::cast_from(288.0_f64) - t8941 / F::cast_from(432.0_f64) + t8944 / F::cast_from(288.0_f64) + F::cast_from(0.60369177012421929545e-2_f64) * t8947 + F::cast_from(0.80492236016562572728e-2_f64) * t1133 * t8953 + F::cast_from(0.18110753103726578864e-2_f64) * t1133 * t8957 + F::cast_from(0.17715845405452227366e4_f64) * t8960 * t8963 - F::cast_from(0.91572784804598301689e1_f64) * t8966 * t8970 + F::cast_from(0.18314556960919660338e2_f64) * t8973 * t8976;
    (t8974, t8975, t8976, t8979)
}
