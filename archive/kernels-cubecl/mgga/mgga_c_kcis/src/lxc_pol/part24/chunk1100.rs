//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1100/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1100<F: Float>(t28931: F, t28964: F, t28995: F, t29022: F, t393: F, t1820: F, t27987: F, t26871: F, t6638: F, t6735: F, t7740: F, t19826: F, t2189: F) -> (F, F, F, F, F, F) {
    let t29024 = t28931 + t28964 + t28995 + t29022;
    let t29025 = t29024 * t393;
    let t29027 = F::cast_from(2.0_f64) * t27987 * t1820;
    let t29029 = F::cast_from(2.0_f64) * t26871 * t6638;
    let t29030 = t7740 * t6735;
    let t29031 = t19826 * t2189;
    (t29024, t29025, t29027, t29029, t29030, t29031)
}
