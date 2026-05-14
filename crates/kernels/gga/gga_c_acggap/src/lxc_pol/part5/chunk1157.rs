//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1157/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1157<F: Float>(t1856: F, t3237: F, t1008: F, t6200: F, t1095: F, t322: F, t384: F, t398: F, t5674: F, t1165: F, t4282: F, t5249: F, t530: F, t1131: F, t1198: F, t14284: F, t14286: F, t14288: F, t1439: F, t1524: F, t1795: F, t1849: F, t336: F, t367: F, t372: F, t4261: F, t4262: F, t4267: F, t4463: F, t4930: F, t5549: F, t5572: F, t8927: F) -> (F,) {
    let t23944 = t3237 * t1856;
    let t23946 = t1008 * t6200;
    let t23951 = t384 * t398 * t1095 * t5674 * t322;
    let t23959 = t4282 * t1165 * t530 * t5249;
    let t23968 = -t4261 * t4262 * t5549 * t372 / 6.0 - t4261 * t4262 * t1849 * t1131 / 12.0 + t4261 * t8927 * t5572 * t372 / 4.0 - t4261 * t4262 * t1439 * t1524 / 6.0 + 0.16006300097412701803e-1 * t23944 + 0.25724410870841842184e-2 * t23946 + 0.85748036236139473944e-3 * t23951 - 0.34299214494455789578e-1 * t4463 * t1165 * t4267 * t4930 - 0.17149607247227894789e-1 * t23959 - t367 * t336 * t1198 * t1795 / 96.0 - 0.17149607247227894789e-2 * t14284 + 0.85748036236139473944e-3 * t14286 - 0.85748036236139473944e-3 * t14288;
    (t23968,)
}
