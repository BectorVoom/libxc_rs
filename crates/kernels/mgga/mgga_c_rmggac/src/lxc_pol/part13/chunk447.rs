//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 447/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk447<F: Float>(t107: F, t622: F, t1656: F, t290: F, t552: F, t839: F, t1602: F, t321: F, t333: F, t559: F, t848: F, t1587: F, t338: F, t352: F, t4616: F, t570: F) -> (F, F, F, F, F, F, F, F) {
    let t5058 = t622 * t107;
    let t5061 = t290 * t1656;
    let t5064 = t552 * t839;
    let t5072 = t1602 * t321;
    let t5076 = t1602 * t333;
    let t5095 = t559 * t848;
    let t5098 = t338 * t1587;
    let t5099 = t5098 * t352;
    let t5102 = t4616 * t570;
    (t5058, t5061, t5064, t5072, t5076, t5095, t5099, t5102)
}
