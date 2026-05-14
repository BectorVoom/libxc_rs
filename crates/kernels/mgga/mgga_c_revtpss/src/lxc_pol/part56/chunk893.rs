//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 893/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk893<F: Float>(t5: F, t34866: F, t8621: F, t33359: F, t33363: F, t33370: F, t33609: F, t33613: F, t33617: F, t33625: F, t8737: F, t8913: F, t117: F, t1518: F, t33346: F, t33640: F, t33642: F, t33644: F, t33646: F, t34453: F, t34455: F, t34457: F, t8564: F) -> (F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t34867 = t8621 * t34866;
    let t34873 = piecewise3(t8, 0.0, 5.0 / 144.0 * t33609 * t8913 - 5.0 / 24.0 * t33359 * t33613 - 5.0 / 36.0 * t33363 * t33617 + 5.0 / 72.0 * t8737 * t34867 + 5.0 / 72.0 * t33370 * t33625);
    let t34874 = t34873 * t117;
    let t34880 = 2.0 * t1518 * t33346 + t33640 + t33642 + t33644 + t33646 + 4.0 * t34453 + 4.0 * t34455 + 4.0 * t34457 + t34874 + t8564;
    (t34867, t34873, t34874, t34880)
}
