//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1059/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1059<F: Float>(t27170: F, t27173: F, t27176: F, t27178: F, t27182: F, t27184: F, t27186: F, t27189: F, t27193: F, t27195: F, t27197: F, t27199: F, t27202: F, t27205: F, t27208: F, t27211: F, t27214: F, t27216: F) -> (F,) {
    let t28114 = -0.26979166666666666666e-1 * t27170 - 0.44965277777777777777e-2 * t27173 - 0.26979166666666666667e-1 * t27176 - 0.44965277777777777777e-2 * t27178 - 0.9375e-1 * t27182 - 0.1875e0 * t27184 - 0.25e0 * t27186 + 0.13489583333333333333e-1 * t27189 + 0.25e0 * t27193 - 0.14388888888888888889e0 * t27195 - 0.1875e0 * t27197 + 0.5e0 * t27199 + 0.125e0 * t27202 + 0.101171875e-1 * t27205 + 0.20833333333333333333e-1 * t27208 + 0.27777777777777777777e-1 * t27211 + 0.1875e0 * t27214 + 0.20234375e-1 * t27216;
    (t28114,)
}
