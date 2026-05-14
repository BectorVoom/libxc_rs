//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 486/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk486<F: Float>(t9097: F, t9100: F, t9108: F, t9111: F, t9113: F, t9115: F, t471: F, t3334: F, t64: F, t2748: F, t871: F, t1365: F, t7906: F, t6525: F, t3355: F, t6313: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10205 = -21.0 / 256.0 * t9097 + 147.0 / 8192.0 * t9100 - 63.0 / 524288.0 * t9108 + 21.0 / 524288.0 * t9111 - 49.0 / 8192.0 * t9113 + 7.0 / 256.0 * t9115;
    let t10206 = t10205 * t471;
    let t10208 = 4.0 / 3.0 * t3334 * t64;
    let t10209 = t2748 * t871;
    let t10211 = 7.0 / 256.0 * t9097;
    let t10212 = 21.0 / 8192.0 * t9100;
    let t10213 = 7.0 / 8192.0 * t9113;
    let t10214 = 7.0 / 768.0 * t9115;
    let t10227 = t1365 * t7906;
    let t10228 = t6525 * t10227;
    let t10229 = 0.11856252764865062333e-2 * t10228;
    let t10236 = 0.7588001769513639893e-1 * t6313 * t3355;
    (t10205, t10206, t10208, t10209, t10211, t10212, t10213, t10214, t10228, t10229, t10236)
}
