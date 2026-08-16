//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1152/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1152<F: Float>(t2122: F, t25146: F, t10309: F, t7565: F, t25163: F, t1923: F, t2123: F, t25102: F, t25110: F, t25114: F, t25117: F, t25120: F, t25150: F, t25159: F, t25162: F, t26749: F, t26755: F, t26783: F, t26786: F, t6954: F, t6960: F, t6963: F, t7566: F, t7576: F, t7579: F) -> (F, F, F, F) {
    let t26789 = t2122 * t25146;
    let t26792 = t10309 * t7565;
    let t26795 = t2122 * t25163;
    let t26798 = F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t26749 * t6960 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t25102 * t2123 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t26755 * t6960 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7566 * t25110 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7566 * t25114 + t25117 * t2123 / F::cast_from(3.0_f64) + t25120 * t2123 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6963 * t7576 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6963 * t7579 - t25150 * t2123 / F::cast_from(6.0_f64) - t6954 * t7576 / F::cast_from(3.0_f64) - t6954 * t7579 / F::cast_from(3.0_f64) - t1923 * t26783 / F::cast_from(6.0_f64) - t1923 * t26786 / F::cast_from(3.0_f64) - t1923 * t26789 / F::cast_from(6.0_f64) - F::cast_from(5.0_f64) * t26792 * t25159 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t25162 * t26795;
    (t26789, t26792, t26795, t26798)
}
