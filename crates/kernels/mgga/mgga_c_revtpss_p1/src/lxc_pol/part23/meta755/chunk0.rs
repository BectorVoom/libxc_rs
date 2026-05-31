//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2545/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2545<F: Float>(t53542: F, t3115: F, t42793: F, t4906: F, t3162: F, t999: F, t42865: F, t72: F, t3088: F, t43472: F, t43401: F, t1062: F, t15655: F) -> (F, F, F, F, F, F, F, F) {
    let t53543 = t53542 / F::cast_from(432.0_f64);
    let t53612 = t3115 * t42793 * t4906;
    let t53613 = F::cast_from(0.14291339372689912324e-3_f64) * t53612;
    let t53619 = t3162 * t999;
    let t53667 = t42865 * t72;
    let t53668 = t3088 * t53667;
    let t53669 = t43472 * t53668;
    let t53676 = t43401 * t53668;
    let t53692 = t15655 * t1062;
    (t53543, t53613, t53619, t53667, t53668, t53669, t53676, t53692)
}
