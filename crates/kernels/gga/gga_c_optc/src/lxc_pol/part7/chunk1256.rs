//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1256/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1256<F: Float>(t24565: F, t25939: F, t25749: F, t3907: F, t3909: F, t25032: F, t953: F, t155: F, t24088: F, t24498: F, t24559: F, t24569: F, t25175: F, t25920: F, t25929: F, t25932: F, t25935: F, t2668: F, t2754: F, t2761: F, t2797: F, t313: F, t314: F, t329: F, t7984: F, t8027: F, t8149: F, t8168: F, t877: F, t914: F, t930: F, t935: F) -> F {
    let t25940 = t25939 * t24565;
    let t25946 = t3907 * t25749 * t3909;
    let t25962 = t953 * t25032;
    let t25964 = F::new(0.24727214904288022343e1) * t2797 * t8027 - F::new(0.1343485452223045261e0) * t25920 + F::new(0.13909058383662012568e1) * t930 * t914 * t25175 + F::new(0.50380704458364197288e-2) * t953 * t24559 + F::new(0.4707813348935102208e4) * t25929 * t2754 - F::new(0.1569271116311700736e4) * t25932 * t2761 - F::new(0.24951672488470492992e3) * t25935 + F::new(0.23184437530160156653e8) * t25940 * t313 * t24569 * t935 + F::new(0.18583473745796456084e3) * t25946 - F::new(0.12117441361606500412e2) * t8149 * t7984 - F::new(0.12117441361606500412e2) * t8149 * t8168 + F::new(0.9291736872898228042e2) * t3907 * t24498 * t3909 - F::new(0.30972456242994093473e2) * t2668 * t24498 * t877 + F::new(0.15599358861923136642e2) * t155 * t329 * t24088 * t314 - F::new(0.11195712101858710508e-1) * t25962;
    t25964
}
