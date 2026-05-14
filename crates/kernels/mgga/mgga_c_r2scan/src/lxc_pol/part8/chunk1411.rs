//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1411/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1411<F: Float>(t2837: F, t5095: F, t9333: F, t7313: F, t8866: F, t20955: F, t2223: F, t25827: F, t25836: F, t25855: F, t30189: F, t30205: F, t30215: F, t30218: F, t30233: F, t32799: F, t33063: F, t33168: F, t506: F, t529: F, t535: F, t538: F) -> (F,) {
    let t34233 = t5095 * t2837 * t9333;
    let t34236 = t7313 * t8866;
    let t34238 = 0.49390868872016336991e0 * t2223 * t529 * t506 * t32799 + 0.49390868872016336991e0 * t2223 * t529 * t506 * t33168 + 0.25426783770825854452e1 * t30189 - 0.27439371595564631661e-1 * t535 * t529 * t538 * t33063 - 0.10401866088065122276e1 * t30205 + 0.98171973930797904389e-1 * t20955 + 0.69861909304693186866e-1 * t30215 + 0.20803732176130244552e1 * t30218 + 0.24393601348456957547e-3 * t30233 + 0.38087975358139160776e-1 * t25827 + 0.20958572791407956061e0 * t34233 + t25836 + 0.95418011034624235142e-2 * t25855 - 0.69345773920434148504e0 * t34236;
    (t34238,)
}
