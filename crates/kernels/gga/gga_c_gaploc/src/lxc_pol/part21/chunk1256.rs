//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1256/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1256<F: Float>(t1890: F, t3487: F, t7805: F, t7810: F, t107: F, t10809: F, t787: F, t20019: F, t7292: F, t11061: F, t14555: F, t5241: F) -> (F, F, F, F) {
    let t33289 = t1890 * t3487;
    let t33291 = t7810 * t33289 * t7805;
    let t33292 = F::new(0.19171462976960374838e1) * t33291;
    let t33294 = t787 * t10809 * t107;
    let t33297 = F::new(0.95334639871601137784e0) * t33294 * t20019 * t7292;
    let t33299 = F::new(0.15337170381568299871e1) * t14555 * t11061;
    let t33308 = t5241 * t3487;
    (t33292, t33297, t33299, t33308)
}
