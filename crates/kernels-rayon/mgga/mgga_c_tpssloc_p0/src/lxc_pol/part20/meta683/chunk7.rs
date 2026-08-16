//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2591/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2591(t225: f64, t52377: f64, t11638: f64, t11720: f64, t11888: f64, t11910: f64, t11914: f64, t11915: f64, t1244: f64, t1246: f64, t1247: f64, t14988: f64, t15245: f64, t15247: f64, t1751: f64, t1755: f64, t23508: f64, t3610: f64, t3624: f64, t3626: f64, t44785: f64, t475: f64, t491: f64, t494: f64, t5068: f64, t5072: f64, t5079: f64, t52424: f64, t52435: f64, t52447: f64, t52458: f64) -> (f64, f64) {
    let t52462 = t52377 * t225;
    let t52471 = -t11720 * t1755 * t23508 * t44785 * t475 + t11638 * t1244 * t1246 * t1751 + t1244 * t1246 * t491 * t52458 - 18.0_f64 * t11888 * t15247 * t5072 + t11914 * t11915 * t52424 + 12.0_f64 * t14988 * t3610 * t5068 - 6.0_f64 * t14988 * t3624 * t5079 - 3.0_f64 * t11910 * t15245 + 3.0_f64 * t1247 * t52447 - 3.0_f64 * t3626 * t52435 + t494 * t52462;
    (t52462, t52471)
}
