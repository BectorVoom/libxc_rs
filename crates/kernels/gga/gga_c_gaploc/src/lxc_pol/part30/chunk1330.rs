//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1330/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1330<F: Float>(t10914: F, t2365: F, t25059: F, t10867: F, t28889: F, t10919: F, t5676: F, t326: F, t32897: F, t825: F, t2684: F, t7585: F) -> (F, F, F, F, F) {
    let t33819 = t10914 * t2365 * t25059;
    let t33820 = F::new(0.89376224879626066674e-1) * t33819;
    let t33823 = t10867 * t28889;
    let t33824 = F::new(0.17875244975925213335e0) * t33823;
    let t33825 = t5676 * t10919;
    let t33826 = F::new(0.59584149919750711116e-1) * t33825;
    let t33829 = F::new(0.18404604457881959845e2) * t825 * t326 * t32897;
    let t33832 = F::new(0.87421871174939309262e2) * t2684 * t7585 * t32897;
    (t33820, t33824, t33826, t33829, t33832)
}
