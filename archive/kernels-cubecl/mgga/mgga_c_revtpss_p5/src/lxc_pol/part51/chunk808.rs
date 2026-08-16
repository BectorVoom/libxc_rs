//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 808/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk808<F: Float>(t1032: F, t4075: F, t545: F, t25875: F, t122: F, t2022: F, t72: F, t3916: F, t2435: F, t7243: F, t555: F, t786: F) -> (F, F, F, F, F, F, F, F) {
    let t25876 = t1032 * t4075;
    let t25877 = t25876 * t545;
    let t25878 = t25875 * t25877;
    let t25880 = t2022 * t72 * t122;
    let t25881 = t25880 * t3916;
    let t25882 = t25878 * t25881;
    let t25893 = F::cast_from(0.73171657588172351096e-2_f64) * t2435 * t7243;
    let t25894 = t786 * t555;
    (t25876, t25877, t25878, t25880, t25881, t25882, t25893, t25894)
}
