//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 762/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk762<F: Float>(t1096: F, t7810: F, t7160: F, t988: F, t7145: F, t4820: F, t7122: F, t4878: F, t7121: F, t4924: F, t7111: F, t1058: F, t7801: F, t1659: F, t7125: F, t1972: F, t4797: F) -> (F, F, F, F, F, F, F, F) {
    let t27440 = t7810 * t1096;
    let t27441 = t7160 * t27440;
    let t27444 = t7810 * t988;
    let t27445 = t7145 * t27444;
    let t27448 = t7122 * t4820;
    let t27450 = t4878 * t7121;
    let t27460 = t7111 * t4924;
    let t27462 = t7801 * t1058;
    let t27464 = t1659 * t7125;
    let t27467 = t4797 * t1972;
    (t27441, t27445, t27448, t27450, t27460, t27462, t27464, t27467)
}
