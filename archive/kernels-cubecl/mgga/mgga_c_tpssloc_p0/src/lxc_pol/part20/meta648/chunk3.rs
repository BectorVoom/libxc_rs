//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2382/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2382<F: Float>(t48096: F, t41831: F, t41833: F, t47707: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47722: F, t47724: F, t47728: F) -> F {
    let t48919 = F::cast_from(0.27385555555555555556e0_f64) * t48096;
    let t48920 = -F::cast_from(0.26574814814814814816e0_f64) * t47707 + F::cast_from(0.39862222222222222222e0_f64) * t47709 + F::cast_from(0.19931111111111111112e0_f64) * t47711 + F::cast_from(0.33218518518518518519e0_f64) * t47713 - F::cast_from(0.11958666666666666667e1_f64) * t47715 - F::cast_from(0.59793333333333333333e0_f64) * t47717 - F::cast_from(0.99655555555555555555e0_f64) * t47722 - F::cast_from(0.11958666666666666667e1_f64) * t47724 - F::cast_from(0.71752000000000000002e1_f64) * t47728 + F::cast_from(0.54771111111111111111e0_f64) * t41831 + F::cast_from(0.32862666666666666666e0_f64) * t41833 - t48919;
    t48920
}
