//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 773/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk773<F: Float>(t1445: F, t35558: F, t813: F, t935: F, t13621: F, t5782: F, t36390: F, t787: F, t9824: F, t10914: F, t2365: F, t35446: F, t13555: F, t4614: F, t833: F, t10811: F, t10978: F) -> (F, F, F, F, F, F) {
    let t45898 = 0.46011511144704899612e1 * t813 * t1445 * t35558 * t935;
    let t45900 = 0.69017266717057349418e1 * t5782 * t13621;
    let t45902 = t787 * t36390 * t9824;
    let t45903 = 0.14896037479937677779e-1 * t45902;
    let t45905 = t10914 * t2365 * t35446;
    let t45906 = 0.89376224879626066674e-1 * t45905;
    let t45913 = 0.15337170381568299871e2 * t833 * t4614 * t13555;
    let t45915 = 0.85801175884441024006e1 * t10811 * t10978;
    (t45898, t45900, t45903, t45906, t45913, t45915)
}
