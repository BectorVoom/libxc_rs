//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 869/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk869(t2060: f64, t2482: f64, t2062: f64, t2823: f64, t5998: f64, t6027: f64, t897: f64, t6029: f64, t4827: f64, t4839: f64, t4996: f64, t5000: f64, t5004: f64, t5008: f64, t7015: f64, t7870: f64) -> f64 {
    let t7872 = t2060 * t2482;
    let t7874 = 0.1350520664e0_f64 * t7872 * t2062;
    let t7876 = 0.1350520664e0_f64 * t2823 * t5998;
    let t7877 = t6027 * t897;
    let t7878 = t7877 * t6029;
    let t7880 = -0.675260332e-1_f64 * t7870 - t7874 - t7876 + 0.1350520664e0_f64 * t7878 - t4996 + t5000 + t5004 + t5008 + t7015 + t4827 - t4839;
    t7880
}
