//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 804/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk804<F: Float>(t30: F, t33: F, t1468: F, t6785: F, t22670: F, t513: F, t5549: F, t5824: F, t9335: F, t1711: F, t6792: F, t516: F, t5557: F, t6416: F, t9350: F, t162: F, t189: F, t512: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t22769 = t6785 * t1468;
    let t22777 = piecewise3(t31, 0.0, -8.0 / 27.0 * t9335 * t22769 + 4.0 / 3.0 * t5549 * t5824 + 4.0 / 3.0 * t513 * t22670);
    let t22778 = t6792 * t1711;
    let t22783 = -t22670;
    let t22787 = piecewise3(t34, 0.0, -8.0 / 27.0 * t9350 * t22778 + 4.0 / 3.0 * t5557 * t6416 + 4.0 / 3.0 * t516 * t22783);
    let t22789 = (t22777 + t22787) * t162;
    let t22790 = t22789 * t189;
    let t22791 = t512 * t22790;
    (t22769, t22778, t22783, t22789, t22791)
}
