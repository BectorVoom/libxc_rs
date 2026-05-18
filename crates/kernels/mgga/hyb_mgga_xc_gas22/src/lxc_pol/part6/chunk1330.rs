//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1330/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1330<F: Float>(t20694: F, t20703: F, t20706: F, t20853: F, t20867: F, t28850: F, t28853: F, t28856: F, t28859: F, t28862: F, t28866: F, t28872: F) -> F {
    let t28996 = F::new(0.3071625e0) * t28850 - F::new(0.59793333333333333334e0) * t28853 + F::new(0.8969e0) * t28856 + F::new(0.39862222222222222223e0) * t28859 + F::new(0.27385555555555555555e0) * t28862 + F::new(0.49294e0) * t28866 + t20867 + F::new(0.27385555555555555556e0) * t20694 + t20853 - F::new(0.18602370370370370371e1) * t20703 + F::new(0.39862222222222222223e0) * t20706 + F::new(0.142419375e1) * t28872;
    t28996
}
