//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1991/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1991<F: Float>(t101522: F, t101761: F, t101767: F, t101980: F, t10416: F, t1312: F, t13435: F, t13440: F, t2055: F, t2322: F, t26153: F, t27123: F, t28219: F, t28683: F, t5523: F, t7373: F, t7889: F, t7983: F, t98484: F, t98487: F) -> F {
    let t102764 = F::new(2.0) * t101522 * t2055 + F::new(2.0) * t101761 * t1312 + F::new(2.0) * t10416 * t7983 + F::new(4.0) * t13435 * t7983 + F::new(2.0) * t13440 * t7983 + F::new(2.0) * t2055 * t98484 + F::new(4.0) * t2055 * t98487 + F::new(4.0) * t2322 * t28683 + F::new(2.0) * t26153 * t7889 + F::new(4.0) * t27123 * t7373 + F::new(4.0) * t28219 * t7373 + F::new(4.0) * t28683 * t5523 + F::new(2.0) * t101767 + t101980;
    t102764
}
