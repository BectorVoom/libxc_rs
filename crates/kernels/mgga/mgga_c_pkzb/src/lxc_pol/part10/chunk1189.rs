//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1189/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1189<F: Float>(t545: F, t6798: F, t83: F, t496: F, t7024: F, t501: F, t6825: F, t1545: F, t2607: F, t2609: F, t5342: F, t5152: F, t114: F, t557: F, t1499: F, t7035: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20325 = t83 * t6798 * t545;
    let t20332 = t496 * t7024;
    let t20334 = t501 * t6825;
    let t20336 = t1545 * t2607;
    let t20340 = t2609 * t5342;
    let t20347 = t501 * t7024;
    let t20353 = t2609 * t5152;
    let t20356 = t6798 * t114 * t557;
    let t20358 = t7035 * t1499;
    (t20325, t20332, t20334, t20336, t20340, t20347, t20353, t20356, t20358)
}
