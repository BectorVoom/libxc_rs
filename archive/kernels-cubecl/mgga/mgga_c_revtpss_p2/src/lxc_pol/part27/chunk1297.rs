//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1297/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1297<F: Float>(t2122: F, t92569: F, t25163: F, t7575: F, t92576: F, t92584: F, t45958: F, t7565: F, t10301: F, t26754: F, t1923: F, t2123: F, t25146: F, t25162: F, t26792: F, t26795: F, t6960: F, t92565: F, t92568: F, t92581: F, t92588: F, t92639: F, t92696: F) -> F {
    let t96752 = t2122 * t92569;
    let t96757 = t7575 * t25163;
    let t96760 = t2122 * t92576;
    let t96765 = t2122 * t92584;
    let t96773 = t45958 * t7565;
    let t96776 = t10301 * t26754;
    let t96779 = -t1923 * t7575 * t25146 / F::cast_from(2.0_f64) + F::cast_from(30.0_f64) * t92568 * t96752 - F::cast_from(10.0_f64) * t92565 * t26795 - F::cast_from(10.0_f64) * t25162 * t96757 - F::cast_from(10.0_f64) * t25162 * t96760 - F::cast_from(15.0_f64) * t26792 * t92581 - F::cast_from(5.0_f64) * t25162 * t96765 - F::cast_from(5.0_f64) * t92588 * t26795 + t92639 * t2123 - F::cast_from(15.0_f64) * t26792 * t92696 + F::cast_from(5.0_f64) / F::cast_from(2.0_f64) * t96773 * t6960 + F::cast_from(5.0_f64) * t96776 * t6960;
    t96779
}
