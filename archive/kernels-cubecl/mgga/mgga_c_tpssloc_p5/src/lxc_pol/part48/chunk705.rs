//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 705/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk705<F: Float>(t22839: F, t3766: F, t556: F, t598: F, t213: F, t1998: F, t236: F, t3734: F, t3872: F, t6952: F, t281: F, t6931: F) -> (F, F, F, F, F) {
    let t22840 = t22839 * t3766;
    let t22842 = t556 * t556;
    let t22843 = F::cast_from(1.0_f64) / t22842;
    let t22844 = t598 * t22843;
    let t22845 = t22844 * t213;
    let t22847 = t1998 * t236 * t3734;
    let t22848 = t22845 * t22847;
    let t22850 = t6952 * t3872;
    let t22852 = t6931 * t281;
    (t22840, t22845, t22848, t22850, t22852)
}
