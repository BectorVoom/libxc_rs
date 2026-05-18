//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1086/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1086<F: Float>(t5: F, t125277: F, t125340: F, t117: F, t125209: F, t116: F, t33629: F, t670: F, t8446: F, t1936: F, t97622: F, t108120: F, t28030: F, t7002: F) -> (F, F, F, F, F, F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t125342 = piecewise3::<f64>(t8, F::new(0.0), t125277 + t125340);
    let t125343 = t125342 * t117;
    let t125344 = F::new(2.0) * t125209;
    let t125345 = t33629 * t116;
    let t125350 = t8446 * t670;
    let t125355 = t97622 * t1936;
    let t125357 = t108120 * t1936;
    let t125359 = t28030 * t7002;
    (t125343, t125344, t125345, t125350, t125355, t125357, t125359)
}
