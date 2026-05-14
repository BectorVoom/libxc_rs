//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1196/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1196<F: Float>(t111979: F, t111984: F, t112090: F, t112153: F, t1196: F, t231: F, t25049: F, t25120: F, t2727: F, t2735: F, t28591: F, t28595: F, t28677: F, t28680: F, t4088: F, t4099: F, t5265: F, t6045: F, t6979: F, t6986: F, t7607: F, t820: F, t98581: F, t98589: F) -> (F,) {
    let t112339 = 0.90613700826057446696e0 * t28591 * t28595 - 0.10947790369858991997e1 * t7607 * t112090 - 0.22653425206514361674e0 * t4099 * t112153 + 0.45306850413028723348e0 * t25120 * t6986 - t98581 + 0.40006800655555555556e0 * t25049 * t6045 * t231 * t4088 * t820 + 0.20003400327777777778e0 * t25049 * t6045 * t231 * t1196 * t2735 + 0.24167761770734866966e0 * t28677 * t111979 - 0.24167761770734866966e0 * t28680 * t111984 - 0.55565000910493827163e-2 * t98589 - 0.54738951849294959988e0 * t5265 * t6979 * t2727;
    (t112339,)
}
