//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 251/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk251<F: Float>(t922: F, t945: F, t26: F, t924: F, t935: F, t937: F, t940: F, t944: F) -> (F, F, F) {
    let t946 = t945 * t922;
    let t947 = t26 * t946;
    let t949 = 0.1898925e1 * t935 - t937 - 0.29896666666666666667e0 * t924 + 0.3071625e0 * t940 - t944 - 0.82156666666666666667e-1 * t947;
    (t946, t947, t949)
}
