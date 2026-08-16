//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1165/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1165(t1745: f64, t981: f64, t14223: f64, t5940: f64, t1851: f64, t3228: f64, t1008: f64, t5546: f64, t5551: f64, t1089: f64, t1173: f64, t12899: f64, t1459: f64, t16008: f64, t16013: f64, t16017: f64, t16023: f64, t16025: f64, t1782: f64, t418: f64, t4680: f64, t5617: f64, t839: f64) -> f64 {
    let t21012 = t981 * t1745;
    let t21014 = t14223 * t5940;
    let t21016 = t3228 * t1851;
    let t21018 = t1008 * t5546;
    let t21020 = t1008 * t5551;
    let t21030 = -0.21437009059034868486e-3_f64 * t12899 - 0.51448821741683684368e-2_f64 * t418 * t1089 * t1459 * t1782 * t839 + 0.42874018118069736972e-3_f64 * t21012 + 0.16006300097412701803e-1_f64 * t21014 + 0.34299214494455789578e-2_f64 * t21016 + 0.68598428988911579156e-2_f64 * t21018 + 0.68598428988911579156e-2_f64 * t21020 - 0.34299214494455789577e-2_f64 * t16008 - 0.17149607247227894789e-2_f64 * t16013 - 0.17149607247227894789e-2_f64 * t16017 - 0.85748036236139473944e-3_f64 * t16023 + 0.34299214494455789578e-2_f64 * t16025 + 0.68598428988911579156e-2_f64 * t1173 * t4680 * t5617;
    t21030
}
