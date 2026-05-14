//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1399/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1399<F: Float>(t10361: F, t1592: F, t2133: F, t2236: F, t29753: F, t29756: F, t29760: F, t29762: F, t29766: F, t29768: F, t33911: F, t33915: F, t33922: F, t33927: F, t33931: F, t33933: F, t360: F, t495: F, t551: F, t552: F, t8629: F, t938: F) -> (F,) {
    let t33938 = 0.39006997830244208535e0 * t1592 * t551 * t552 * t938 * t8629 + 0.34672886960217074253e0 * t33911 - 0.43341108700271342816e-1 * t2236 * t10361 - 0.41607464352260489104e1 * t33915 + 0.12713391885412927226e1 * t29753 + 0.40752780427737692339e0 * t29756 - 0.4939086887201633699e-1 * t29760 + 0.17563392970889009434e0 * t29762 - 0.49390868872016336988e-1 * t29766 - 0.48787202696913915093e-2 * t33922 + 0.19756347548806534795e0 * t29768 - 0.1047928639570397803e0 * t33927 - 0.20958572791407956061e0 * t33931 + 0.43341108700271342816e-1 * t2133 * t360 * t33933 * t495;
    (t33938,)
}
