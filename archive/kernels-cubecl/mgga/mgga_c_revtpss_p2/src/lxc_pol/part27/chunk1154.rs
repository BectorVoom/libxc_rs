//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1154/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1154<F: Float>(t2139: F, t3655: F, t1256: F, t7610: F, t2138: F, t3670: F, t3666: F, t3678: F, t7613: F, t3685: F, t7607: F, t3596: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t26821 = F::cast_from(0.95275595817932748827e-4_f64) * t2139 * t3655;
    let t26822 = t7610 * t1256;
    let t26824 = t3670 * t2138;
    let t26827 = t3666 * t2138;
    let t26832 = t7613 * t3678;
    let t26836 = t7607 * t3685;
    let t26842 = t3596 * sigma2;
    (t26821, t26822, t26824, t26827, t26832, t26836, t26842)
}
