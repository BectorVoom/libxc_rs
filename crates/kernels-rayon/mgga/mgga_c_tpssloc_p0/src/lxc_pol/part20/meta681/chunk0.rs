//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2567/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2567(t1254: f64, t3633: f64, t1157: f64, t1164: f64, t14829: f64, t3375: f64, t14966: f64, t3378: f64, t15823: f64, t225: f64, t15800: f64, t15808: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51906 = t1254 * t3633;
    let t51913 = 0.35089341735807877242e1_f64 * t1164 * t3375 * t14829 * t1157;
    let t51916 = 0.10526802520742363173e2_f64 * t1164 * t14966 * t3378;
    let t51925 = t15823 * t225;
    let t51928 = t15800 * t225;
    let t51937 = t15808 * t225;
    (t51906, t51913, t51916, t51925, t51928, t51937)
}
