//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1355/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1355<F: Float>(t300: F, t3880: F, t931: F, t10107: F, t3174: F, t68: F, t10111: F, t3874: F, t3199: F, t824: F, t17938: F, t10063: F, t10106: F, t18992: F, t2185: F, t22979: F, t2371: F, t2888: F, t3175: F, t3185: F, t3206: F, t3898: F, t3913: F, t406: F, t6366: F, t6367: F, t6470: F, t6518: F, t6526: F, t7945: F, t8254: F, t8255: F, t8259: F, t8270: F, t8278: F, t8428: F, t8435: F, t8451: F) -> (F, F, F, F, F) {
    let t27020 = t300 * t931 * t3880;
    let t27028 = t3174 * t68 * t10107;
    let t27031 = t3174 * t68 * t10111;
    let t27044 = t300 * t931 * t3874;
    let t27057 = t824 * t3199;
    let t27062 = t3874 * t17938;
    let t27071 = 0.85748036236139473944e-3 * t3206 * t27020 * t8255 - t18992 / 216.0 - 2.0 / 9.0 * t10063 * t8278 - t27028 / 24.0 + t27031 / 36.0 - t3174 * t2888 * t10106 * t2185 / 16.0 + t3174 * t2888 * t3175 * t7945 / 24.0 - t10063 * t8270 / 9.0 - 0.51448821741683684367e-2 * t8428 * t27044 * t6518 * t8259 + 0.51448821741683684367e-2 * t8435 * t27044 * t6526 * t8259 + 0.85748036236139473944e-3 * t3206 * t8254 * t6470 * t3898 - 0.34299214494455789578e-2 * t3185 * t8254 * t2371 * t27057 - 0.21437009059034868486e-3 * t22979 * t406 * t27062 * t8451 - 0.12862205435420921092e-2 * t3206 * t6366 * t3913 * t6367;
    (t27020, t27044, t27057, t27062, t27071)
}
