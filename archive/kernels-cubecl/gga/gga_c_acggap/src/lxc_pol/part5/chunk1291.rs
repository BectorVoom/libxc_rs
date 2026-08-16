//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1291/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1291<F: Float>(t1131: F, t1165: F, t1198: F, t14284: F, t14286: F, t14288: F, t1439: F, t1524: F, t1795: F, t1849: F, t23944: F, t23946: F, t23951: F, t23959: F, t336: F, t367: F, t372: F, t4261: F, t4262: F, t4267: F, t4463: F, t4930: F, t5549: F, t5572: F, t8927: F) -> F {
    let t23968 = -t4261 * t4262 * t5549 * t372 / F::cast_from(6.0_f64) - t4261 * t4262 * t1849 * t1131 / F::cast_from(12.0_f64) + t4261 * t8927 * t5572 * t372 / F::cast_from(4.0_f64) - t4261 * t4262 * t1439 * t1524 / F::cast_from(6.0_f64) + F::cast_from(0.16006300097412701803e-1_f64) * t23944 + F::cast_from(0.25724410870841842184e-2_f64) * t23946 + F::cast_from(0.85748036236139473944e-3_f64) * t23951 - F::cast_from(0.34299214494455789578e-1_f64) * t4463 * t1165 * t4267 * t4930 - F::cast_from(0.17149607247227894789e-1_f64) * t23959 - t367 * t336 * t1198 * t1795 / F::cast_from(96.0_f64) - F::cast_from(0.17149607247227894789e-2_f64) * t14284 + F::cast_from(0.85748036236139473944e-3_f64) * t14286 - F::cast_from(0.85748036236139473944e-3_f64) * t14288;
    t23968
}
