//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 797/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk797<F: Float>(t2823: F, t6001: F, t2060: F, t2482: F, t2062: F, t5998: F, t6027: F, t897: F, t6029: F, t2055: F, t2056: F, t955: F) -> (F, F, F, F, F) {
    let t7870 = t2823 * t6001;
    let t7872 = t2060 * t2482;
    let t7874 = F::cast_from(0.1350520664e0_f64) * t7872 * t2062;
    let t7876 = F::cast_from(0.1350520664e0_f64) * t2823 * t5998;
    let t7877 = t6027 * t897;
    let t7878 = t7877 * t6029;
    let t7898 = t2055 * t955 * t2056;
    (t7870, t7874, t7876, t7878, t7898)
}
