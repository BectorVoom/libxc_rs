//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 838/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk838<F: Float>(t5202: F, t5205: F, t5209: F, t5212: F, t5213: F, t5218: F, t5220: F, t5225: F, t5230: F, t5233: F, t5237: F, t3128: F, t60: F) -> (F, F) {
    let t8884 = -t5202 - t5205 - t5209 + t5212 + F::new(0.26680286006577777776e-2) * t5213 - t5218 - F::new(0.33872559466666666666e-2) * t5220 - t5225 + t5230 - t5233 - F::new(0.10843581300301739842e-1) * t5237;
    let t8892 = t60 * t3128;
    (t8884, t8892)
}
