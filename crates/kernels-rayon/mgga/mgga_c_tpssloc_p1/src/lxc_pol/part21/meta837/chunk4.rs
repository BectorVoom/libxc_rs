//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2982/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2982(t14080: f64, t4571: f64, t14202: f64, t4644: f64, t1043: f64, t1615: f64, t375: f64, t10408: f64, t1041: f64, t1044: f64, t10957: f64, t10965: f64, t14229: f64, t17890: f64, t248: f64, t2771: f64, t2780: f64, t3070: f64, t3071: f64, t3117: f64, t42721: f64, t49822: f64, t49827: f64, t49829: f64, t49831: f64, t49846: f64, t5857: f64, t5861: f64, t5867: f64, t59682: f64, t59690: f64, t62064: f64) -> (f64, f64) {
    let t62282 = t14080 * t4571;
    let t62284 = t4644 * t14202;
    let t62291 = t375 * t1043 * t1615;
    let t62296 = t49822 / 1152.0_f64 - t42721 / 6912.0_f64 + t3070 * t3071 * t5867 * t2780 / 4608.0_f64 + 5.0_f64 / 13824.0_f64 * t3070 * t10408 * t5867 * t2771 + 19.0_f64 / 1944.0_f64 * t49827 - t49829 / 324.0_f64 + t49831 / 648.0_f64 + t3117 * t17890 / 2304.0_f64 + t1041 * t248 * t1044 * t59682 / 4608.0_f64 + 5.0_f64 / 13824.0_f64 * t10965 * t5861 + 19.0_f64 / 2592.0_f64 * t10957 * t5857 - t62282 / 324.0_f64 - t62284 / 10368.0_f64 - t1041 * t248 * t1044 * t59690 / 1152.0_f64 + t62064 * t62291 * t14229 / 576.0_f64 - 5.0_f64 / 1728.0_f64 * t49846;
    (t62291, t62296)
}
