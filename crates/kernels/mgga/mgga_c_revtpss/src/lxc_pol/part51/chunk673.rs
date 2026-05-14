//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 673/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk673<F: Float>(t1940: F, t30: F, t8490: F, t8494: F, t1032: F, t1982: F, t359: F, t365: F, t369: F) -> (F, F, F, F, F) {
    let t8498 = t1940 * t8490 * t30 / 2.0 - t1940 * t8494 * t30 / 2.0;
    let t8499 = t1982 * t1032;
    let t8500 = t359 * t365;
    let t8501 = t8500 * t369;
    let t8502 = t8499 * t8501;
    (t8498, t8499, t8500, t8501, t8502)
}
