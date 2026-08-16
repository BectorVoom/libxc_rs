//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1358/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1358<F: Float>(t10365: F, t4953: F, t1445: F, t1562: F, t26428: F, t874: F, t10488: F, t1457: F, t31585: F, t475: F) -> (F, F, F, F) {
    let t34216 = F::cast_from(0.13803453343411469884e2_f64) * t4953 * t10365;
    let t34220 = F::cast_from(0.69017266717057349418e1_f64) * t1562 * t1445 * t26428 * t874;
    let t34223 = t1457 * t10488;
    let t34239 = t31585 * t475;
    (t34216, t34220, t34223, t34239)
}
