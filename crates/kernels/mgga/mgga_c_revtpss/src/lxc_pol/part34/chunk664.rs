//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 664/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk664<F: Float>(t30: F, t33: F, t1468: F, t3833: F, t513: F, t5824: F, t1711: F, t3841: F, t516: F, t6416: F, t162: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t6785 = t1468 * t1468;
    let t6791 = piecewise3::<F>(t31, F::new(0.0), F::new(4.0) / F::new(9.0) * t3833 * t6785 + F::new(4.0) / F::new(3.0) * t513 * t5824);
    let t6792 = t1711 * t1711;
    let t6798 = piecewise3::<F>(t34, F::new(0.0), F::new(4.0) / F::new(9.0) * t3841 * t6792 + F::new(4.0) / F::new(3.0) * t516 * t6416);
    let t6800 = (t6791 + t6798) * t162;
    (t6785, t6792, t6800)
}
