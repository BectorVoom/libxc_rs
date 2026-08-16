//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2090/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2090<F: Float>(t2435: F, t27986: F, t1904: F, t2439: F, t25916: F, t26050: F, t27884: F, t25304: F, t27883: F, t25946: F, t25898: F, t97699: F) -> (F, F, F, F, F) {
    let t97792 = t2435 * t27986;
    let t97795 = t2439 * t25916 * t1904;
    let t97798 = F::cast_from(0.25702851531048074406e-1_f64) * t27884 * t26050;
    let t97799 = t25304 * t27883;
    let t97800 = t97799 * t25946;
    let t97802 = t97699 * t25898;
    (t97792, t97795, t97798, t97800, t97802)
}
