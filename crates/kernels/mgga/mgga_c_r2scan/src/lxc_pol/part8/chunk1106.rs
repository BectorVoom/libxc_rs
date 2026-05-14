//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1106/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1106<F: Float>(t1414: F, t23: F, t254: F, t255: F, t6077: F, t6311: F, t6321: F, t2168: F, t6217: F, t545: F, t6534: F, t1567: F, t489: F, t146: F, t252: F, t6322: F, t776: F) -> (F, F, F, F, F, F) {
    let t20253 = 0.20211424382067871469e1 * t254 * t6311 / t23 / t6077 / t1414 * t255 * t6321;
    let t20286 = t6217 * t2168;
    let t20298 = t545 * t6534;
    let t20303 = t489 * t1567;
    let t20305 = t146 * t20303 * t252;
    let t20328 = t776 * t6322;
    (t20253, t20286, t20298, t20303, t20305, t20328)
}
