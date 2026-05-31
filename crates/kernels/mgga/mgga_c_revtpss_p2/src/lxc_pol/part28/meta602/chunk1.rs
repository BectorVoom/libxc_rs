//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2080/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2080<F: Float>(t18227: F, t7003: F, t25861: F, t4248: F, t3813: F, t651: F, t7741: F, t116: F, t28159: F, t18153: F, t1936: F, t670: F, t6982: F) -> (F, F, F, F, F, F) {
    let t97608 = F::cast_from(4.0_f64) * t18227 * t7003;
    let t97610 = F::cast_from(4.0_f64) * t4248 * t25861;
    let t97617 = F::cast_from(2.0_f64) * t651 * t3813 * t7741;
    let t97622 = t28159 * t116;
    let t97629 = F::cast_from(2.0_f64) * t651 * t18153 * t1936;
    let t97632 = t6982 * t670;
    (t97608, t97610, t97617, t97622, t97629, t97632)
}
