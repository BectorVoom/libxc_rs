//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1028/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1028(t1081: f64, t116481: f64, t119691: f64, t119713: f64, t123418: f64, t123719: f64, t123745: f64, t123757: f64, t123764: f64, t1877: f64, t23788: f64, t24191: f64, t24339: f64, t2522: f64, t25901: f64, t25905: f64, t25927: f64, t25928: f64, t25930: f64, t25934: f64, t25938: f64, t25945: f64, t26739: f64, t26756: f64, t28: f64, t32030: f64, t32034: f64, t32047: f64, t33991: f64, t34052: f64, t6841: f64, t6848: f64, t7114: f64, t7649: f64, t7844: f64, t8744: f64) -> f64 {
    let t123938 = 3.0_f64 / 2.0_f64 * t2522 * t8744 * t25901 - t1877 * t123719 * t6848 / 2.0_f64 - t1877 * t24339 * t34052 - t1877 * t7114 * t28 * t26739 + t1877 * t32047 * t25945 + 3.0_f64 / 2.0_f64 * t2522 * t8744 * t25938 - 3.0_f64 * t123757 * t119691 + 3.0_f64 * t116481 * t119713 - 3.0_f64 * t24191 * t23788 * t123745 + 3.0_f64 / 2.0_f64 * t2522 * t32030 * t7649 + 2.0_f64 * t26756 * t25927 * t123418 + t123764 * t25928 + t1877 * t33991 * t1081 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t33991 * t6841 - t1877 * t32034 * t25945 / 2.0_f64 - t1877 * t7114 * t1081 * t7844 + 3.0_f64 / 2.0_f64 * t2522 * t8744 * t25905 + t1877 * t32047 * t25930 + t1877 * t32047 * t25934;
    t123938
}
