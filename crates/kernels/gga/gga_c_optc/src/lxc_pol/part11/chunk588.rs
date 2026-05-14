//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 588/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk588<F: Float>(t3634: F, t4947: F, t1375: F, t3788: F, t4919: F, t828: F, t837: F, t845: F, t2472: F, t4854: F, t2476: F, t4783: F, t4785: F, t4817: F, t4821: F, t4858: F, t4897: F, t4900: F, t4927: F) -> (F, F, F, F, F, F, F, F) {
    let t4948 = t3634 * t4947;
    let t4952 = 0.11696446794910408142e1 * t3788 * t1375;
    let t4954 = t828 * t4919 * t837;
    let t4956 = 0.58482233974552040708e0 * t845 * t4954;
    let t4957 = t2472 * t4854;
    let t4958 = t4957 * t2476;
    let t4960 = 0.17315755899375863299e2 * t845 * t4958;
    let t4961 = -t4897 + t4785 - t4900 + t4817 + t4821 + t4927 + t4783 - t4952 + t4858 - t4956 - t4960;
    (t4948, t4952, t4954, t4956, t4957, t4958, t4960, t4961)
}
