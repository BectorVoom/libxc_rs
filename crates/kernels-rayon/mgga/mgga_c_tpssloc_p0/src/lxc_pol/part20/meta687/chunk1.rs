//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2603/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2603(t11651: f64, t15507: f64, t11709: f64, t1174: f64, t11741: f64, t1177: f64, t11805: f64, t11809: f64, t15622: f64, t15627: f64, t15631: f64, t1737: f64, t3506: f64, t44858: f64, t44896: f64, t45080: f64, t4582: f64, t4978: f64, t5005: f64, t50865: f64, t50869: f64, t52659: f64, t52836: f64) -> f64 {
    let t52845 = t15507 * t11651;
    let t52853 = t11709 * t15622 / 512.0_f64 + 3.0_f64 / 512.0_f64 * t44896 * t15627 - 3.0_f64 / 512.0_f64 * t44858 * t15631 + t3506 * t4582 * t52659 * t4978 / 512.0_f64 + t52836 * t11741 / 3072.0_f64 - t5005 * t11805 / 4608.0_f64 - t5005 * t11809 / 768.0_f64 + t45080 * t1737 / 3072.0_f64 + t52845 / 288.0_f64 - t1174 * t1177 * t50865 / 48.0_f64 - t1174 * t1177 * t50869 / 16.0_f64;
    t52853
}
