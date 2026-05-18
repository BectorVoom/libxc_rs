//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1298/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1298<F: Float>(t2247: F, t26781: F, t38: F, t2123: F, t25102: F, t25110: F, t25114: F, t26749: F, t26755: F, t6960: F, t7566: F, t7576: F, t7579: F, t92654: F, t92658: F, t92709: F, t92711: F) -> F {
    let t96792 = t2247 * t38 * t26781;
    let t96803 = F::new(5.0) * t26749 * t25110 + F::new(5.0) / F::new(2.0) * t26749 * t25114 + t92709 * t2123 + t92711 * t2123 + F::new(2.0) * t25102 * t7576 + F::new(2.0) * t25102 * t7579 + F::new(5.0) / F::new(2.0) * t96792 * t6960 + F::new(5.0) * t26755 * t25110 + F::new(5.0) / F::new(2.0) * t26755 * t25114 + F::new(5.0) / F::new(2.0) * t7566 * t92654 + F::new(5.0) / F::new(2.0) * t7566 * t92658;
    t96803
}
