//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 877/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk877<F: Float>(t1614: F, t664: F, t40331: F, t793: F, t2298: F, t26531: F, t558: F, t7817: F, t797: F, t305: F, t38381: F, t326: F, t333: F, t40575: F, t40918: F, t40922: F, t40925: F, t40930: F, t40934: F, t40938: F, t5155: F) -> (F, F, F) {
    let t40940 = t664 * t1614;
    let t40944 = t793 * t40331;
    let t40946 = t26531 * t2298;
    let t40948 = t7817 * t558;
    let t40949 = t797 * t40948;
    let t40951 = t305 * t38381;
    let t40953 = -0.54549323308490683456e-1 * t40918 + 0.36366215538993788971e0 * t40922 - 0.81823984962736025184e-1 * t40925 - 0.40911992481368012593e-1 * t40930 + 0.54549323308490683457e-1 * t40934 - 0.11974241701863808564e0 * t326 * t40575 - 0.44903406381989282115e-1 * t40938 + 0.47896966807455234256e0 * t5155 * t40940 * t333 + 0.2927036860455597649e0 * t40944 - 0.8980681276397856423e-1 * t40946 - 0.43905552906833964735e0 * t40949 - 0.14635184302277988245e0 * t40951;
    (t40940, t40948, t40953)
}
