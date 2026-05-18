//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1199/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1199<F: Float>(t3382: F, t5895: F, t5899: F, t1173: F, t1180: F, t1181: F, t13261: F, t13264: F, t16755: F, t16757: F, t16759: F, t1894: F, t3396: F, t407: F, t4267: F, t4680: F, t4757: F, t5270: F, t5862: F, t5894: F, t6119: F, t930: F) -> F {
    let t21801 = t3382 * t5895;
    let t21815 = t3382 * t5899;
    let t21825 = F::new(0.12862205435420921092e-2) * t13261 + F::new(0.17149607247227894789e-2) * t13264 - F::new(0.80031500487063509016e-2) * t16755 + F::new(0.20007875121765877254e-2) * t16757 - F::new(0.85748036236139473944e-3) * t21801 + F::new(0.16006300097412701803e-1) * t16759 - F::new(0.85748036236139473944e-3) * t1180 * t4680 * t5894 - F::new(0.85748036236139473944e-3) * t1180 * t1181 * t6119 * t407 - F::new(0.42874018118069736972e-3) * t1180 * t1181 * t1894 * t930 + F::new(0.85748036236139473944e-3) * t21815 + F::new(0.34299214494455789578e-2) * t1173 * t1181 * t5862 * t5270 + F::new(0.13719685797782315831e-1) * t3396 * t1181 * t4267 * t4757;
    t21825
}
