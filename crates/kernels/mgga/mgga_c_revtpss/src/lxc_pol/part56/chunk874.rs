//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 874/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk874<F: Float>(t32790: F, t33549: F, t118: F, t1310: F, t2127: F, t2163: F, t32118: F, t32123: F, t32131: F, t32182: F, t32299: F, t32320: F, t32338: F, t32340: F, t32856: F, t32858: F, t32862: F, t32864: F, t32867: F, t33375: F, t508: F, t649: F, t7584: F, t7683: F, t8917: F, t8964: F) -> (F, F) {
    let t33550 = t32790 + t33549;
    let t33552 = -t118 * t33550 - t1310 * t8917 - 2.0 * t2127 * t7683 - 2.0 * t2163 * t7584 - t33375 * t508 - t649 * t8964 - t32118 - t32123 + t32131 + t32182 + t32299 - t32320 - t32338 - t32340 - 4.0 * t32856 - 4.0 * t32858 - 4.0 * t32862 - 4.0 * t32864 - 4.0 * t32867;
    (t33550, t33552)
}
