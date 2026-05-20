//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1037/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1037<F: Float>(t1936: F, t7474: F, t651: F, t7374: F, t8634: F, t2055: F, t7221: F, t649: F, t8686: F, t1937: F, t26399: F, t28658: F) -> (F, F, F, F, F, F, F) {
    let t32401 = t7474 * t1936;
    let t32402 = t651 * t32401;
    let t32404 = t8634 * t7374;
    let t32410 = t7221 * t2055;
    let t32415 = t649 * t8686;
    let t32417 = F::new(2.0) * t26399 * t1937;
    let t32419 = F::new(2.0) * t28658 * t1937;
    (t32401, t32402, t32404, t32410, t32415, t32417, t32419)
}
