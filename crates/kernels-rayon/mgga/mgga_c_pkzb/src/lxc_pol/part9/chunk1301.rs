//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1301/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1301(t18854: f64, t2252: f64, t2259: f64, t22795: f64, t22815: f64, t22822: f64, t22825: f64, t22826: f64, t22829: f64, t22837: f64, t22840: f64, t22844: f64, t22847: f64, t22851: f64, t3103: f64, t6269: f64, t6272: f64, t6303: f64, t6314: f64, t8068: f64, t8107: f64, t8132: f64, t8135: f64, t863: f64, t871: f64) -> f64 {
    let t22856 = 3.0_f64 * t6303 * t3103 + 3.0_f64 * t2252 * t8068 + 1.0_f64 * t863 * (t22795 + t22815) * t871 + t22822 + t22825 - 6.0_f64 * t22826 * t2259 - 0.19298375398431042081e3_f64 * t22829 * t6314 + 0.35089341735807877242e1_f64 * t8107 * t6269 - t22837 - t22840 - t22844 - t22847 - t22851 - 6.0_f64 * t6272 * t8132 - 0.57895126195293126242e3_f64 * t18854 * t8135;
    t22856
}
