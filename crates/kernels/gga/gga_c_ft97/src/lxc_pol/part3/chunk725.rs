//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 725/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk725<F: Float>(t11264: F, t11296: F, t11299: F, t11304: F, t12527: F, t15837: F, t15840: F, t15843: F, t8074: F, t8110: F, t8914: F, t15847: F, t15850: F, t15852: F, t15855: F, t15858: F, t15861: F, t15863: F, t15866: F, t15869: F, t15872: F, t15876: F) -> (F, F) {
    let t16812 = 0.29634667152263374488e-1 * t8074 + t8914 - 0.3704333394032921811e-2 * t8110 - 0.29634667152263374488e-1 * t11296 - 0.7408666788065843622e-2 * t11299 + 0.14817333576131687244e-1 * t11264 - t12527 + 0.22226000364197530866e-1 * t11304 - 0.59269334304526748973e-1 * t15837 + 0.74086667880658436217e-2 * t15840 + 0.51860667516460905352e-1 * t15843;
    let t16824 = -0.13335600218518518519e0 * t15847 + 0.8890400145679012346e-1 * t15850 + 0.88904001456790123461e-1 * t15852 - 0.11113000182098765433e-1 * t15855 + 0.10001700163888888889e0 * t15858 - 0.13335600218518518519e0 * t15861 - 0.44452000728395061731e-1 * t15863 + 0.55565000910493827163e-2 * t15866 + 0.22226000364197530865e-1 * t15869 - 0.33339000546296296298e-1 * t15872 + 0.16669500273148148149e-1 * t15876;
    (t16812, t16824)
}
