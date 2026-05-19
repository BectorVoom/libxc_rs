//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1052/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1052<F: Float>(t34735: F, t8650: F, t1356: F, t36646: F, t36663: F, t36674: F, t3928: F, t40791: F, t41790: F, t41792: F, t41796: F, t41803: F, t41808: F, t41812: F, t41813: F, t41818: F, t41822: F, t41829: F, t41834: F, t41836: F, t5226: F, t665: F) -> F {
    let t41838 = t34735 * t8650;
    let t41840 = t41790 + t41792 - F::cast_from(0.11974241701863808564e0_f64) * t36646 + F::cast_from(0.1064114997332445985e-4_f64) * t41796 - F::cast_from(0.1064114997332445985e-4_f64) * t41803 - F::cast_from(0.85129199786595678796e-5_f64) * t41808 - t41812 + F::cast_from(0.59590439850616975156e-4_f64) * t41813 + t41818 + t41822 - F::cast_from(0.19863479950205658386e-4_f64) * t36663 - F::cast_from(0.30487649791575028314e-3_f64) * t36674 + F::cast_from(0.35922725105591425692e0_f64) * t3928 * t665 * t5226 - t41829 + F::cast_from(0.79828278012425390428e-1_f64) * t1356 * t40791 + F::cast_from(0.25538759935978703638e-4_f64) * t41834 - F::cast_from(0.81823984962736025184e-1_f64) * t41836 - F::cast_from(0.20455996240684006296e-1_f64) * t41838;
    t41840
}
