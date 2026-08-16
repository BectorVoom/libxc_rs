//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 884/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk884<F: Float>(t2122: F, t25163: F, t2139: F, t3655: F, t1256: F, t7610: F, t2138: F, t3666: F, t3678: F, t7613: F, t3685: F, t7607: F) -> (F, F, F, F, F, F) {
    let t26795 = t2122 * t25163;
    let t26821 = F::cast_from(0.95275595817932748827e-4_f64) * t2139 * t3655;
    let t26822 = t7610 * t1256;
    let t26827 = t3666 * t2138;
    let t26832 = t7613 * t3678;
    let t26836 = t7607 * t3685;
    (t26795, t26821, t26822, t26827, t26832, t26836)
}
