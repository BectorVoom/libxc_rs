//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 629/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk629(t10972: f64, t1445: f64, t813: f64, t3477: f64, t5771: f64, t10713: f64, t1457: f64, t2103: f64, t10717: f64, t3470: f64, t8478: f64, t8638: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10973 = t1445 * t10972;
    let t10975 = 0.46011511144704899612e1_f64 * t813 * t10973;
    let t10977 = 0.71500979903700853338e0_f64 * t5771 * t3477;
    let t10978 = t1457 * t10713;
    let t10980 = 0.71500979903700853338e0_f64 * t2103 * t10978;
    let t10981 = t1457 * t10717;
    let t10983 = 0.71500979903700853338e0_f64 * t2103 * t10981;
    let t10988 = 0.10725146985555128001e1_f64 * t8478 * t3470;
    let t10990 = 0.10725146985555128001e1_f64 * t8638 * t3470;
    (t10975, t10977, t10980, t10983, t10988, t10990)
}
