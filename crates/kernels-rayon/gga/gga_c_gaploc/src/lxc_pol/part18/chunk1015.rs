//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1015/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1015(t10964: f64, t813: f64, t10783: f64, t1457: f64, t2194: f64, t3484: f64, t8528: f64, t935: f64, t1445: f64, t3477: f64, t5771: f64, t10713: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10966 = 0.61348681526273199483e1_f64 * t813 * t10964;
    let t10967 = t1457 * t10783;
    let t10971 = 0.46011511144704899612e1_f64 * t2194 * t3484;
    let t10972 = t8528 * t935;
    let t10973 = t1445 * t10972;
    let t10975 = 0.46011511144704899612e1_f64 * t813 * t10973;
    let t10977 = 0.71500979903700853338e0_f64 * t5771 * t3477;
    let t10978 = t1457 * t10713;
    (t10966, t10967, t10971, t10972, t10973, t10975, t10977, t10978)
}
