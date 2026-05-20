//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 981/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk981<F: Float>(t33591: F, t651: F, t5542: F, t8595: F, t2014: F, t1868: F, t4147: F, t32119: F, t1937: F, t28030: F, t1518: F, t1931: F) -> (F, F, F, F, F, F, F) {
    let t33592 = t651 * t33591;
    let t33594 = t8595 * t5542;
    let t33595 = t2014 * t33594;
    let t33596 = t4147 * t1868;
    let t33597 = t32119 * t33596;
    let t33599 = F::new(3.0) * t2014 * t33597;
    let t33600 = t28030 * t1937;
    let t33602 = t1931 * t1518;
    (t33592, t33594, t33595, t33597, t33599, t33600, t33602)
}
