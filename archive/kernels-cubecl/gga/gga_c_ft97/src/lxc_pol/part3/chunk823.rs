//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 823/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk823<F: Float>(t11264: F, t11296: F, t11299: F, t11304: F, t12527: F, t15837: F, t15840: F, t15843: F, t8074: F, t8110: F, t8914: F, t15847: F, t15850: F, t15852: F, t15855: F, t15858: F, t15861: F, t15863: F, t15866: F, t15869: F, t15872: F, t15876: F) -> (F, F) {
    let t16812 = F::cast_from(0.29634667152263374488e-1_f64) * t8074 + t8914 - F::cast_from(0.3704333394032921811e-2_f64) * t8110 - F::cast_from(0.29634667152263374488e-1_f64) * t11296 - F::cast_from(0.7408666788065843622e-2_f64) * t11299 + F::cast_from(0.14817333576131687244e-1_f64) * t11264 - t12527 + F::cast_from(0.22226000364197530866e-1_f64) * t11304 - F::cast_from(0.59269334304526748973e-1_f64) * t15837 + F::cast_from(0.74086667880658436217e-2_f64) * t15840 + F::cast_from(0.51860667516460905352e-1_f64) * t15843;
    let t16824 = -F::cast_from(0.13335600218518518519e0_f64) * t15847 + F::cast_from(0.8890400145679012346e-1_f64) * t15850 + F::cast_from(0.88904001456790123461e-1_f64) * t15852 - F::cast_from(0.11113000182098765433e-1_f64) * t15855 + F::cast_from(0.10001700163888888889e0_f64) * t15858 - F::cast_from(0.13335600218518518519e0_f64) * t15861 - F::cast_from(0.44452000728395061731e-1_f64) * t15863 + F::cast_from(0.55565000910493827163e-2_f64) * t15866 + F::cast_from(0.22226000364197530865e-1_f64) * t15869 - F::cast_from(0.33339000546296296298e-1_f64) * t15872 + F::cast_from(0.16669500273148148149e-1_f64) * t15876;
    (t16812, t16824)
}
