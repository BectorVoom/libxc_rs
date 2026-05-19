//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 936/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk936<F: Float>(t13648: F, t2194: F, t1445: F, t35558: F, t813: F, t935: F, t13621: F, t5782: F, t36390: F, t787: F, t9824: F, t10914: F, t2365: F, t35446: F) -> (F, F, F, F, F) {
    let t45894 = F::cast_from(0.46011511144704899612e1_f64) * t2194 * t13648;
    let t45898 = F::cast_from(0.46011511144704899612e1_f64) * t813 * t1445 * t35558 * t935;
    let t45900 = F::cast_from(0.69017266717057349418e1_f64) * t5782 * t13621;
    let t45902 = t787 * t36390 * t9824;
    let t45903 = F::cast_from(0.14896037479937677779e-1_f64) * t45902;
    let t45905 = t10914 * t2365 * t35446;
    (t45894, t45898, t45900, t45903, t45905)
}
