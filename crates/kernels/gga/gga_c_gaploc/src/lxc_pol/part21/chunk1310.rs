//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1310/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1310<F: Float>(t123: F, t7861: F, t883: F, t2487: F, t2488: F, t2464: F, t2465: F, t7995: F, t31585: F, t447: F, t6716: F, t6717: F) -> (F, F, F, F, F) {
    let t34363 = t7861 * t123 * t883;
    let t34365 = t2487 * t2488 * t34363;
    let t34366 = F::new(0.19171462976960374838e0) * t34365;
    let t34369 = t2487 * t2464 * t2465 * t7995;
    let t34370 = F::new(0.85206502119823888168e-1) * t34369;
    let t34371 = t31585 * t447;
    let t34374 = F::new(0.13803453343411469884e2) * t6716 * t6717 * t34371;
    (t34363, t34366, t34370, t34371, t34374)
}
