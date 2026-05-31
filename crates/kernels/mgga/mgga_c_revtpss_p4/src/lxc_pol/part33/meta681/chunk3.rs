//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2224/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2224<F: Float>(t116: F, t30715: F, t108078: F, t108080: F, t108083: F, t108085: F, t108087: F, t108089: F, t108099: F, t108103: F, t108105: F, t108107: F, t108109: F, t108111: F, t108117: F, t1843: F, t29422: F, t29456: F, t30944: F, t4248: F, t4292: F, t649: F, t651: F, t671: F, t7732: F, t8233: F) -> (F, F) {
    let t111696 = t30715 * t116;
    let t111704 = -F::cast_from(4.0_f64) * t4292 * t651 * t8233 - F::cast_from(2.0_f64) * t111696 * t671 - F::cast_from(2.0_f64) * t1843 * t29422 - F::cast_from(4.0_f64) * t29456 * t4248 - F::cast_from(4.0_f64) * t29456 * t7732 - t30944 * t649 - t108078 - t108080 - t108083 - t108085 - t108087 - t108089 - t108099 + t108103 - t108105 - t108107 - t108109 - t108111 - t108117;
    (t111696, t111704)
}
