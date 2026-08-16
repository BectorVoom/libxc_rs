//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1330/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1330(t10914: f64, t2365: f64, t25059: f64, t10867: f64, t28889: f64, t10919: f64, t5676: f64, t326: f64, t32897: f64, t825: f64, t2684: f64, t7585: f64) -> (f64, f64, f64, f64, f64) {
    let t33819 = t10914 * t2365 * t25059;
    let t33820 = 0.89376224879626066674e-1_f64 * t33819;
    let t33823 = t10867 * t28889;
    let t33824 = 0.17875244975925213335e0_f64 * t33823;
    let t33825 = t5676 * t10919;
    let t33826 = 0.59584149919750711116e-1_f64 * t33825;
    let t33829 = 0.18404604457881959845e2_f64 * t825 * t326 * t32897;
    let t33832 = 0.87421871174939309262e2_f64 * t2684 * t7585 * t32897;
    (t33820, t33824, t33826, t33829, t33832)
}
