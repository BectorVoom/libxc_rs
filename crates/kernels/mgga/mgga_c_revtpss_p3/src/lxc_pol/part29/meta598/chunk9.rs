//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2036/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2036<F: Float>(t100993: F, t100997: F, t101035: F, t101040: F, t101051: F, t101061: F, t101070: F, t101074: F, t103554: F, t103561: F, t1113: F, t1940: F, t2071: F, t2403: F, t25752: F, t25767: F, t26425: F, t26590: F, t27773: F, t28291: F, t28456: F, t33: F, t4541: F, t51780: F, t7428: F, t8020: F, t8046: F) -> F {
    let t103817 = F::new(3.0) * t4541 * t2071 * t101070 - F::new(3.0) * t26425 * t101061 + t1940 * t26590 * t101040 + F::new(3.0) / F::new(2.0) * t2403 * t8020 * t25767 - t103561 + F::new(3.0) * t4541 * t8020 * t25752 + F::new(3.0) * t28291 * t101035 + F::new(3.0) * t2403 * t7428 * t27773 + t1940 * t103554 * t33 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t2403 * t2071 * t100993 + F::new(3.0) * t2403 * t2071 * t100997 + F::new(3.0) * t51780 * t8046 + F::new(3.0) / F::new(2.0) * t2403 * t2071 * t101051 + F::new(3.0) / F::new(2.0) * t2403 * t2071 * t101074 + t1940 * t28456 * t1113;
    t103817
}
