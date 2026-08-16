//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1730/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1730(t25: f64, t265: f64, t394: f64, t26806: f64, t1409: f64, t2064: f64, t26775: f64, t3966: f64, t40: f64, t607: f64, t7131: f64, t7865: f64, t1081: f64, t1649: f64, t1877: f64, t2057: f64, t24191: f64, t24339: f64, t2522: f64, t25892: f64, t25898: f64, t25901: f64, t25905: f64, t25921: f64, t25928: f64, t25930: f64, t25934: f64, t25938: f64, t25945: f64, t26563: f64, t26740: f64, t26744: f64, t26756: f64, t26774: f64, t28: f64, t6841: f64, t6848: f64, t7110: f64, t7114: f64, t7649: f64, t7656: f64, t7845: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t26807 = piecewise3(t395, 0.0_f64, t26806);
    let t26814 = piecewise3(t115, t26775, t7131 * t1409 / 2.0_f64 + t2064 * t3966 / 2.0_f64 + t26807 * t40 / 2.0_f64 + t7865 * t607 / 2.0_f64);
    let t26861 = 3.0_f64 * t26563 * t25892 + 3.0_f64 / 2.0_f64 * t2522 * t7110 * t7649 - 3.0_f64 / 2.0_f64 * t24191 * t25898 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t25901 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t25905 + 3.0_f64 / 2.0_f64 * t2522 * t7845 * t6841 + t1877 * t26740 * t28 / 2.0_f64 - t1877 * t26744 * t6848 / 2.0_f64 + t1877 * t7845 * t1081 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t24191 * t25921 - t1877 * t24339 * t7656 / 2.0_f64 + t26756 * t25928 - t1877 * t7114 * t25930 / 2.0_f64 - t1877 * t7114 * t25934 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t25938 + t1877 * t7110 * t1649 / 2.0_f64 - t1877 * t7114 * t25945 / 2.0_f64 - t26774;
    (t26807, t26814, t26861)
}
