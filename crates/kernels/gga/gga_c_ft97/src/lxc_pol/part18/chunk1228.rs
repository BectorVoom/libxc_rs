//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1228/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1228<F: Float>(t11960: F, t5710: F, t6557: F, t8360: F, t5502: F, t8216: F, t10969: F, t22506: F, t22503: F, t100127: F, t11397: F, t1286: F, t1538: F, t1588: F, t22883: F, t22935: F, t25574: F, t25584: F, t25612: F, t28: F, t5510: F, t5620: F, t6562: F, t94002: F, t94019: F, t984: F) -> (F, F, F, F, F) {
    let t102372 = t5710 * t11960;
    let t102376 = t8360 * t6557;
    let t102385 = t8216 * t5502;
    let t102392 = t10969 * t22506;
    let t102394 = t10969 * t22503;
    let t102399 = -2.0 * t102372 - t22935 * t25574 / 9.0 - 2.0 * t102376 + t25584 * t5620 / 3.0 + t1286 * t28 * t22883 * t984 * t1588 - t94002 / 18.0 - 4.0 / 9.0 * t100127 * t102385 * t11397 - t1538 * t6562 - 2.0 / 3.0 * t25584 * t5510 + 4.0 * t102392 + 8.0 * t102394 + 2.0 / 9.0 * t22935 * t25612 - t94019 / 3.0;
    (t102372, t102376, t102392, t102394, t102399)
}
