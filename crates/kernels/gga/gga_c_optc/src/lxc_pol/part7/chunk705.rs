//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 705/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk705<F: Float>(t43: F, t50: F, t1891: F, t47: F, t6534: F, t6541: F, t6713: F, t6716: F, t99: F, t1896: F, t553: F, t1900: F, t52: F, t6548: F, t6554: F, zeta_threshold: F) -> (F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t6722 = piecewise3::<F>(t44, F::new(0.0), -F::new(8.0) / F::new(27.0) * t6713 * t6534 + F::new(4.0) / F::new(3.0) * t6716 * t1891 + F::new(4.0) / F::new(3.0) * t47 * t6541);
    let t6724 = F::new(1.0) / t99 / t50;
    let t6727 = t1896 * t553;
    let t6733 = piecewise3::<F>(t51, F::new(0.0), -F::new(8.0) / F::new(27.0) * t6724 * t6548 + F::new(4.0) / F::new(3.0) * t6727 * t1900 + F::new(4.0) / F::new(3.0) * t52 * t6554);
    (t6722, t6724, t6727, t6733)
}
