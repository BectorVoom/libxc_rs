//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 772/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk772<F: Float>(t1312: F, t1518: F, t2055: F, t4248: F, t7359: F, t7889: F, t7969: F, t7983: F, t7488: F, t7900: F, t7499: F, t7501: F, t7502: F, t7504: F, t7904: F, t7906: F, t7908: F) -> (F, F, F) {
    let t8075 = F::new(2.0) * t1312 * t7983 + F::new(2.0) * t1518 * t7359 + F::new(2.0) * t2055 * t4248 + F::new(2.0) * t2055 * t7889 + t7969;
    let t8079 = t7488 * t7900;
    let t8085 = -t7499 - t7904 / F::new(24.0) - t7501 + t7502 - F::new(0.85748036236139473944e-3) * t7906 - t7504 - F::new(0.34299214494455789578e-2) * t7908;
    (t8075, t8079, t8085)
}
