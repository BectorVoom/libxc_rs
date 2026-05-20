//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 934/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk934<F: Float>(t117: F, t22746: F, t1312: F, t1518: F, t18245: F, t22633: F, t22639: F, t4248: F, t5920: F, t7889: F, t13584: F, t22186: F) -> (F, F, F, F) {
    let t22747 = t22746 * t117;
    let t22758 = F::new(2.0) * t1312 * t22633 + F::new(6.0) * t1518 * t18245 + F::new(6.0) * t4248 * t5920 + F::new(6.0) * t5920 * t7889 + F::new(6.0) * t22639 + t22747;
    let t22762 = F::new(60.0) * t13584;
    let t22763 = F::cast_from(0.54934341918019635162e-3_f64) * t22186;
    (t22747, t22758, t22762, t22763)
}
