//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1143/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1143<F: Float>(t1165: F, t1173: F, t12720: F, t12724: F, t12728: F, t12734: F, t12736: F, t12739: F, t12744: F, t12748: F, t12750: F, t15550: F, t15560: F, t1748: F, t175: F, t20175: F, t397: F, t398: F) -> F {
    let t20517 = F::new(0.60023625365297631762e-2) * t12720 - F::new(0.42874018118069736972e-3) * t397 * t398 * t175 * t20175 + F::new(0.20007875121765877254e-2) * t12724 + F::new(0.85748036236139473944e-3) * t12728 - F::new(0.12004725073059526352e-1) * t15550 + F::new(0.25724410870841842184e-2) * t12734 + F::new(0.42874018118069736972e-3) * t12736 + F::new(0.17149607247227894789e-2) * t1173 * t1165 * t15560 * t1748 + F::new(0.40015750243531754508e-2) * t12739 + F::new(0.45351183609335988442e-1) * t12744 - F::new(0.22675591804667994222e-1) * t12748 + F::new(0.22675591804667994222e-1) * t12750;
    t20517
}
