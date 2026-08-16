//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1089/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1089<F: Float>(t242: F, t2675: F, t4978: F, t2722: F, t1407: F, t3950: F, t2741: F, t1465: F, t3758: F, t4989: F, t837: F, t4994: F) -> (F, F, F, F, F) {
    let t15027 = t242 * t2675 * t4978;
    let t15028 = t2722 * t15027;
    let t15031 = t3950 * t1407;
    let t15032 = t2741 * t15031;
    let t15035 = t1465 * t3758;
    let t15036 = t2741 * t15035;
    let t15039 = t4989 * t837;
    let t15040 = t2741 * t15039;
    let t15043 = t4994 * t837;
    (t15028, t15032, t15036, t15040, t15043)
}
