//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 916/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk916(t5: f64, t32110: f64, t7687: f64, t1458: f64, t8774: f64, t15899: f64, t8808: f64, t1441: f64, t8717: f64, t3701: f64, t7939: f64, t2095: f64, t32245: f64, t32249: f64, t32257: f64, t32258: f64, t33103: f64, t33107: f64, t33111: f64, t33119: f64, t8707: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t33878 = t32110 * t7687;
    let t33883 = t8774 * t1458;
    let t33886 = t8808 * t15899;
    let t33893 = t1441 * t8717;
    let t33899 = t3701 * t7939;
    let t33900 = t2095 * t33899;
    let t33915 = piecewise3(t8, 0.0_f64, 5.0_f64 / 36.0_f64 * t33103 * t8707 - 5.0_f64 / 6.0_f64 * t32245 * t33107 - 5.0_f64 / 9.0_f64 * t32249 * t33111 - t32257 + 5.0_f64 / 18.0_f64 * t32258 * t33119);
    (t33878, t33883, t33886, t33893, t33899, t33900, t33915)
}
