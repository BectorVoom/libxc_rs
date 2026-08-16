//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 866/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk866(t10570: f64, t35828: f64, t1486: f64, t193: f64, t2781: f64, t35833: f64, t1234: f64, t7611: f64, t852: f64, t6308: f64, t33819: f64, t33846: f64, t35822: f64, t35826: f64, t35831: f64, t35836: f64, t35840: f64, t35844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35846 = t10570 * t35828;
    let t35848 = t1486 * t193 * t35846;
    let t35849 = t2781 * t35833;
    let t35851 = t1486 * t193 * t35849;
    let t35853 = t7611 * t1234;
    let t35854 = t852 * t35853;
    let t35856 = t6308 * t193 * t35854;
    let t35858 = t35822 / 2.0_f64 + t33819 + 2.0_f64 / 9.0_f64 * t35826 + 4.0_f64 / 3.0_f64 * t35831 - 2.0_f64 / 3.0_f64 * t35836 - t35840 / 6.0_f64 - t33846 - t35844 / 9.0_f64 - t35848 + 2.0_f64 / 3.0_f64 * t35851 + t35856 / 12.0_f64;
    (t35846, t35848, t35849, t35851, t35853, t35854, t35856, t35858)
}
