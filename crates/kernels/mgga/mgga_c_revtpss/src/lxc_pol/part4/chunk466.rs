//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 466/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk466<F: Float>(t1610: F, t915: F, t1594: F, t939: F, t1601: F, t1604: F, t1607: F, t948: F, t951: F, t954: F) -> (F, F, F, F) {
    let t1612 = F::new(1.0) * t915 * t1610;
    let t1614 = -t939 - F::new(0.17123333333333333333e-1) * t1594;
    let t1621 = F::new(0.3529725e1) * t1601 - t948 - F::new(0.516475e0) * t1594 + F::new(0.6311625e0) * t1604 - t951 - F::new(0.104195e0) * t1607;
    let t1622 = t1621 * t954;
    (t1612, t1614, t1621, t1622)
}
