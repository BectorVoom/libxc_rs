//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1312/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1312<F: Float>(t2201: F, t2687: F, t6416: F, t1551: F, t20116: F, t20122: F, t20125: F, t20134: F, t20139: F, t2185: F, t24774: F, t24777: F, t24778: F, t24788: F, t24792: F, t24795: F, t2526: F, t2634: F, t5109: F, t551: F, t552: F, t6218: F, t6583: F) -> (F,) {
    let t24798 = t2201 * t6416 * t2687;
    let t24802 = 0.24393601348456957547e-3 * t20116 - 0.7801399566048841707e0 * t6218 * t551 * t552 * t2526 * t2185 + 0.24393601348456957546e-3 * t24774 - t24777 + 0.69345773920434148506e0 * t24778 + 0.2037639021386884617e0 * t20122 - 0.4075278042773769234e0 * t20125 - 0.26004665220162805689e0 * t6583 * t5109 * t2634 * t1551 + 0.52396431978519890151e-1 * t24788 + 0.20958572791407956061e0 * t24792 + 0.29272321618148349056e-1 * t24795 - 0.17465477326173296717e-1 * t24798 - 0.57131963037208741167e-1 * t20134 + 0.5141876673348786705e0 * t20139;
    (t24802,)
}
