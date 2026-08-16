//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1903/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1903<F: Float>(t2453: F, t3908: F, t8086: F, t28829: F, t689: F, t25899: F, t26271: F, t27884: F, t28862: F, t686: F, t72: F, t25895: F) -> (F, F, F, F, F, F) {
    let t102266 = t2453 * t8086 * t3908;
    let t102268 = t28829 * t689;
    let t102270 = F::cast_from(0.25702851531048074406e-1_f64) * t25899 * t102268;
    let t102272 = F::cast_from(0.25702851531048074406e-1_f64) * t27884 * t26271;
    let t102274 = t28862 * t72 * t686;
    let t102276 = F::cast_from(0.28912093960683998208e-1_f64) * t25895 * t102274;
    (t102266, t102268, t102270, t102272, t102274, t102276)
}
