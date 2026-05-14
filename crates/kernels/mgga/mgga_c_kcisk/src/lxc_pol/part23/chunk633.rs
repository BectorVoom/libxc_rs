//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 633/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk633<F: Float>(t4176: F, t4183: F, t4186: F, t4190: F, t4194: F, t4198: F, t4201: F, t4206: F, t4212: F, t4216: F, t4218: F, t4220: F, t4224: F, t4227: F, t4233: F, t4238: F, t4242: F, t4298: F, t4302: F, t4307: F, t4310: F, t4314: F, t4316: F, t4318: F) -> (F, F) {
    let t4551 = 0.625e-1 * t4176 - 0.34173611111111111111e0 * t4183 + 0.14388888888888888889e0 * t4186 + 0.101171875e-1 * t4190 - 0.13489583333333333333e-1 * t4194 - 0.9375e-1 * t4198 + 0.5e0 * t4201 - 0.125e0 * t4206 + 0.1875e0 * t4212 - 0.1875e0 * t4216 + 0.10791666666666666667e0 * t4218 - 0.26979166666666666666e-1 * t4220;
    let t4564 = 0.20234375e-1 * t4224 - 0.10791666666666666667e0 * t4227 + 0.26979166666666666666e-1 * t4233 - 0.20234375e-1 * t4238 - 0.20833333333333333333e-1 * t4242 + 0.9375e-1 * t4298 - 0.101171875e-1 * t4302 - 0.44965277777777777777e-2 * t4307 - 0.33333333333333333334e0 * t4310 + 0.91666666666666666667e0 * t4314 - 0.5e0 * t4316 + 0.125e0 * t4318;
    (t4551, t4564)
}
