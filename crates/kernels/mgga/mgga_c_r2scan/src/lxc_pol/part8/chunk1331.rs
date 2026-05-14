//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1331/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1331<F: Float>(t2591: F, t32532: F, t10050: F, t2133: F, t2294: F, t6106: F, t9998: F, t10106: F, t10111: F, t20127: F, t2122: F, t2124: F, t22721: F, t2551: F, t2557: F, t2573: F, t27753: F, t27763: F, t27786: F, t27814: F, t27820: F, t27823: F, t32516: F, t32523: F, t495: F, t5108: F, t5109: F, t7337: F) -> (F, F) {
    let t32533 = t32532 * t2591;
    let t32546 = t2133 * t2294 * t10050;
    let t32551 = t6106 * t2294 * t9998;
    let t32554 = 0.76280351312477563357e1 * t27753 - 0.27439371595564631661e-1 * t2557 * t2124 * t32516 * t495 - 0.76830240467580968652e0 * t27763 + 0.38415120233790484326e0 * t27786 + 0.78013995660488417067e0 * t22721 * t5109 * t32523 * t2573 + 0.78013995660488417067e0 * t20127 * t5109 * t32523 * t2551 - 0.32927245914677557992e0 * t2122 * t7337 * t32533 - 0.39006997830244208535e0 * t5108 * t5109 * t10106 * t495 - 0.39006997830244208535e0 * t5108 * t5109 * t10111 * t495 - 0.34672886960217074253e0 * t32546 - 0.11524536070137145298e1 * t27814 - 0.76830240467580968651e0 * t27820 + 0.41607464352260489104e1 * t32551 + 0.41607464352260489103e1 * t27823;
    (t32533, t32554)
}
