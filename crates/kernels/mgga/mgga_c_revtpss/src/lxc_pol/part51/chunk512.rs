//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 512/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk512<F: Float>(t5: F, t1497: F, t2242: F, t2247: F, t4171: F, t4173: F, t4178: F, t4241: F, t603: F, t644: F, t91: F, t117: F, t116: F, t1501: F) -> (F, F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t4245 = piecewise3::<f64>(t8, F::new(0.0), -F::new(4.0) * t1497 * t2242 + F::new(20.0) * t2247 * t4178 + t4171 * t91 - F::new(4.0) * t4173 * t644 - F::new(4.0) * t4241 * t603);
    let t4246 = t4245 * t117;
    let t4248 = t1501 * t116;
    (t4245, t4246, t4248)
}
