//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1008/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1008<F: Float>(t117: F, t34873: F, t1518: F, t33346: F, t33640: F, t33642: F, t33644: F, t33646: F, t34453: F, t34455: F, t34457: F, t8564: F) -> (F, F) {
    let t34874 = t34873 * t117;
    let t34880 = F::cast_from(2.0_f64) * t1518 * t33346 + t33640 + t33642 + t33644 + t33646 + F::cast_from(4.0_f64) * t34453 + F::cast_from(4.0_f64) * t34455 + F::cast_from(4.0_f64) * t34457 + t34874 + t8564;
    (t34874, t34880)
}
