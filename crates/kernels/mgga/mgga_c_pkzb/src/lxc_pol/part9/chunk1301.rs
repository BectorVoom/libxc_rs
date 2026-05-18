//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1301/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1301<F: Float>(t18854: F, t2252: F, t2259: F, t22795: F, t22815: F, t22822: F, t22825: F, t22826: F, t22829: F, t22837: F, t22840: F, t22844: F, t22847: F, t22851: F, t3103: F, t6269: F, t6272: F, t6303: F, t6314: F, t8068: F, t8107: F, t8132: F, t8135: F, t863: F, t871: F) -> F {
    let t22856 = F::new(3.0) * t6303 * t3103 + F::new(3.0) * t2252 * t8068 + F::new(1.0) * t863 * (t22795 + t22815) * t871 + t22822 + t22825 - F::new(6.0) * t22826 * t2259 - F::new(0.19298375398431042081e3) * t22829 * t6314 + F::new(0.35089341735807877242e1) * t8107 * t6269 - t22837 - t22840 - t22844 - t22847 - t22851 - F::new(6.0) * t6272 * t8132 - F::new(0.57895126195293126242e3) * t18854 * t8135;
    t22856
}
