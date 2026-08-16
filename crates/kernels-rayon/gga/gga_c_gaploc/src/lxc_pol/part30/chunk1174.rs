//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1174/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1174(t10249: f64, t6305: f64, t2268: f64, t31585: f64, t426: f64, t535: f64, t10119: f64, t10232: f64, t10590: f64, t1063: f64, t1306: f64, t1324: f64, t1358: f64, t1365: f64, t31652: f64, t31655: f64, t31660: f64, t31662: f64, t31672: f64, t31674: f64, t31679: f64, t3371: f64, t3808: f64, t3822: f64, t3833: f64, t448: f64) -> f64 {
    let t31681 = 0.68292015925622759036e0_f64 * t6305 * t10249;
    let t31685 = 0.56910013271352299198e-1_f64 * t2268 * t535 * t31585 * t426;
    let t31686 = -t31652 + 0.63233348079280332442e-2_f64 * t3808 * t10232 + 0.63233348079280332442e-2_f64 * t1358 * t1365 * t31655 - t31660 + t31662 - 0.56910013271352299198e-1_f64 * t3833 * t10119 - 0.56910013271352299198e-1_f64 * t1063 * t10590 * t448 - 0.28455006635676149599e-1_f64 * t1063 * t3371 * t1306 - t31672 - t31674 + 0.56910013271352299198e-1_f64 * t3822 * t3371 * t1324 + t31679 + t31681 + t31685;
    t31686
}
