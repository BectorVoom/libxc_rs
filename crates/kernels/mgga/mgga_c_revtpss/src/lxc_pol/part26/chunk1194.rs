//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1194/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1194<F: Float>(t10489: F, t10627: F, t10818: F, t11054: F, t11061: F, t14365: F, t1940: F, t198: F, t207: F, t2070: F, t2071: F, t2394: F, t2403: F, t2408: F, t2430: F, t26581: F, t26585: F, t26590: F, t2832: F, t41161: F, t4541: F, t50066: F, t51775: F, t51792: F, t51806: F, t7428: F, t7432: F, t775: F, t890: F, t892: F, t95527: F, t95953: F, t95964: F, t95976: F) -> F {
    let t96072 = t198 * t207 * t95953 * t892 + F::new(3.0) * t2403 * t2071 * t10489 - t1940 * t7432 * t11054 + F::new(18.0) * t2403 * t26590 * t50066 + F::new(9.0) * t2403 * t7428 * t2430 - F::new(18.0) * t4541 * t7432 * t51775 + F::new(18.0) * t4541 * t2071 * t10818 - F::new(3.0) * t1940 * t26585 * t2832 - F::new(18.0) * t2403 * t26585 * t14365 - F::new(3.0) * t1940 * t95527 * t890 - F::new(9.0) * t2403 * t7432 * t51806 - F::new(9.0) * t2403 * t7432 * t41161 + F::new(9.0) * t2403 * t26581 * t775 + F::new(6.0) * t198 * t10627 * t2070 * t892 + F::new(18.0) * t4541 * t7428 * t2394 - F::new(6.0) * t1940 * t95964 * t11061 + F::new(6.0) * t1940 * t26590 * t51792 + F::new(6.0) * t1940 * t95976 * t2408;
    t96072
}
