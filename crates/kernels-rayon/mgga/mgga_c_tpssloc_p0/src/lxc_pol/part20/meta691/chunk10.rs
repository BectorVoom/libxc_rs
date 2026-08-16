//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2633/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2633(t11712: f64, t11913: f64, t491: f64, t11887: f64, t52834: f64, t11616: f64, t11640: f64, t11890: f64, t11907: f64, t11914: f64, t15022: f64, t15023: f64, t15240: f64, t15241: f64, t15248: f64, t15429: f64, t1758: f64, t3604: f64, t3624: f64, t44691: f64, t45323: f64, t5064: f64, t5072: f64, t5075: f64, t5079: f64, t5080: f64) -> (f64, f64) {
    let t53545 = t11712 * t11913 * t491;
    let t53565 = t52834 * t11887;
    let t53590 = 3.0_f64 * t11914 * t15429 * t5072 + 3.0_f64 * t11914 * t15429 * t5075 - 3.0_f64 * t15022 * t3624 * t5072 - 3.0_f64 * t15240 * t3624 * t5079 + t11616 * t1758 + t11640 * t5064 - 6.0_f64 * t11890 * t53565 - 3.0_f64 * t11907 * t15023 + 3.0_f64 * t15241 * t3604 - 18.0_f64 * t15248 * t44691 - 3.0_f64 * t45323 * t5080;
    (t53545, t53590)
}
