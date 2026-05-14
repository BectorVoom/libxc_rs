//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1117/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1117<F: Float>(t33291: F, t107: F, t10809: F, t787: F, t20019: F, t7292: F, t11061: F, t14555: F, t3487: F, t5241: F, t23000: F, t7805: F, t28279: F, t3040: F, t28435: F, t28811: F) -> (F, F, F, F, F, F, F) {
    let t33292 = 0.19171462976960374838e1 * t33291;
    let t33294 = t787 * t10809 * t107;
    let t33297 = 0.95334639871601137784e0 * t33294 * t20019 * t7292;
    let t33299 = 0.15337170381568299871e1 * t14555 * t11061;
    let t33308 = t5241 * t3487;
    let t33310 = t23000 * t33308 * t7805;
    let t33311 = 0.11502877786176224903e1 * t33310;
    let t33313 = 0.71500979903700853338e0 * t28279 * t3040;
    let t33315 = 0.35750489951850426669e0 * t28435 * t3040;
    let t33317 = 0.71500979903700853338e0 * t28811 * t3040;
    (t33292, t33297, t33299, t33311, t33313, t33315, t33317)
}
