//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1381/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1381<F: Float>(t1848: F, t5480: F, t1856: F, t5465: F, t1665: F, t6458: F, t21947: F, t550: F, t21984: F, t546: F, t1673: F, t20649: F, t3: F, t4544: F, t67858: F, t67860: F, t67868: F, t67874: F, t67879: F, t72724: F) -> F {
    let t72750 = t1848 * t5480;
    let t72751 = t5465 * t1856;
    let t72752 = t1665 * t6458;
    let t72754 = t21947 * t550;
    let t72755 = t546 * t21984;
    let t72756 = t3 * t550 * t72724 + F::cast_from(2.0_f64) * t1673 * t20649 + F::cast_from(2.0_f64) * t4544 * t6458 + t67858 + t67860 + t67868 + t67874 + t67879 + t72750 + t72751 + F::cast_from(2.0_f64) * t72752 + t72754 + t72755;
    t72756
}
