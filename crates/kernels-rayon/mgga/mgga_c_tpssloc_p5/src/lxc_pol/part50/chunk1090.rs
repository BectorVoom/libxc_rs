//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1090/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1090(t32734: f64, t32780: f64, t533: f64, t1390: f64, t1983: f64, t1442: f64, t1849: f64, t1869: f64, t1976: f64, t32656: f64, t32659: f64, t32661: f64, t32664: f64, t32666: f64, t32668: f64, t32671: f64, t32674: f64, t32676: f64, t32679: f64, t32680: f64, t32684: f64, t6517: f64, t652: f64, t7451: f64, t7472: f64, t7670: f64, t8329: f64, t8439: f64, t8447: f64) -> (f64, f64, f64, f64) {
    let t32781 = t32734 + t32780;
    let t32782 = t533 * t32781;
    let t32783 = t32782 * t1390;
    let t32784 = t1983 * t32783;
    let t32785 = -t1442 * t8439 + t1849 * t8447 - 2.0_f64 * t1869 * t7670 - 2.0_f64 * t1976 * t7451 - 2.0_f64 * t32656 * t652 - 4.0_f64 * t6517 * t7472 - 4.0_f64 * t32659 - 4.0_f64 * t32661 - 4.0_f64 * t32664 - t32666 + 6.0_f64 * t32668 - 4.0_f64 * t32671 - t32674 - t32676 - t32679 - 4.0_f64 * t32680 + t32684 + t32784 - t8329;
    (t32781, t32782, t32783, t32785)
}
