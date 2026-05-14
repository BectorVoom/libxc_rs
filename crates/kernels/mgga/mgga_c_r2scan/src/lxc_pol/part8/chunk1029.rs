//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1029/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1029<F: Float>(t5202: F, t5205: F, t5209: F, t5212: F, t5218: F, t5225: F, t5230: F, t5233: F, t5248: F, t5263: F, t7647: F, t7650: F, t7662: F, t7664: F, t7667: F, t7669: F, t7671: F) -> (F,) {
    let t10207 = -t5202 - t5205 - t5209 + t5212 - t5218 - t5225 + t5230 - t5233 + 0.57791679765211885292e1 * t7647 + 0.5143752e0 * t7650 - 0.31168546390226634765e3 * t7662 + 0.10526802520742363173e2 * t7664 - 0.1016176784e-1 * t7667 + 0.21687162600603479684e-1 * t7669 - 60.0 * t7671 + t5248 + t5263;
    (t10207,)
}
