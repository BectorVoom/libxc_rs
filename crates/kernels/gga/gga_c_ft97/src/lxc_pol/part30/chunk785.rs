//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 785/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk785<F: Float>(t35858: F, t36001: F, t312: F, t6353: F, t7124: F, t1248: F, t34012: F, t1501: F, t28859: F, t1212: F, t1506: F, t6222: F, t193: F, t1253: F, t7612: F, t34031: F, t34036: F, t35822: F, t35826: F, t35831: F, t35836: F, t35840: F, t35844: F, t35848: F, t35851: F, t35856: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t36002 = t35858 + t36001;
    let t36003 = t36002 * t312;
    let t36005 = t6353 * t7124;
    let t36007 = t34012 * t1248;
    let t36009 = t28859 * t1501;
    let t36011 = t1506 * t1212;
    let t36012 = t6222 * t36011;
    let t36013 = t193 * t36012;
    let t36016 = t7612 * t1253;
    let t36017 = t193 * t36016;
    let t36033 = 3.0 / 2.0 * t35822 + t34031 + 2.0 / 3.0 * t35826 + 4.0 * t35831 - 2.0 * t35836 - t35840 / 2.0 - t34036 - t35844 / 3.0 - 3.0 * t35848 + 2.0 * t35851 + t35856 / 4.0;
    (t36002, t36003, t36005, t36007, t36009, t36011, t36012, t36013, t36016, t36017, t36033)
}
