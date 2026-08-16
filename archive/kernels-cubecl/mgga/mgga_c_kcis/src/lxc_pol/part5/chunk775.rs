//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 775/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk775<F: Float>(t41: F, t556: F, t5676: F, t6027: F, t1529: F, t2047: F, t1547: F, t2061: F, t1546: F, t5627: F, t572: F, t1533: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6028 = t41 * t556;
    let t6029 = t6028 * t5676;
    let t6030 = t6027 * t6029;
    let t6032 = t1529 * t2047;
    let t6034 = t2061 * t1547;
    let t6035 = t1546 * t6034;
    let t6037 = t556 * t5627;
    let t6038 = t572 * t6037;
    let t6039 = t1533 * t6038;
    (t6028, t6029, t6030, t6032, t6034, t6035, t6037, t6038, t6039)
}
