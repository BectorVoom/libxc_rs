//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1850/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1850<F: Float>(t2122: F, t25163: F, t1923: F, t2123: F, t25102: F, t25110: F, t25114: F, t25117: F, t25120: F, t25150: F, t25159: F, t25162: F, t26749: F, t26755: F, t26783: F, t26786: F, t26789: F, t26792: F, t6954: F, t6960: F, t6963: F, t7566: F, t7576: F, t7579: F) -> (F, F) {
    let t26795 = t2122 * t25163;
    let t26798 = F::new(5.0) / F::new(3.0) * t26749 * t6960 + F::new(2.0) / F::new(3.0) * t25102 * t2123 + F::new(5.0) / F::new(3.0) * t26755 * t6960 + F::new(5.0) / F::new(3.0) * t7566 * t25110 + F::new(5.0) / F::new(6.0) * t7566 * t25114 + t25117 * t2123 / F::new(3.0) + t25120 * t2123 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t6963 * t7576 + F::new(2.0) / F::new(3.0) * t6963 * t7579 - t25150 * t2123 / F::new(6.0) - t6954 * t7576 / F::new(3.0) - t6954 * t7579 / F::new(3.0) - t1923 * t26783 / F::new(6.0) - t1923 * t26786 / F::new(3.0) - t1923 * t26789 / F::new(6.0) - F::new(5.0) * t26792 * t25159 - F::new(10.0) / F::new(3.0) * t25162 * t26795;
    (t26795, t26798)
}
