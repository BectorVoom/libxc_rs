//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1195/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1195<F: Float>(t1808: F, t9833: F, t7842: F, t7848: F, t9839: F, t9838: F, t2970: F, t9834: F, t1815: F, t19: F, t3918: F, t10087: F, t1184: F, t1804: F, t1824: F, t1863: F, t1997: F, t23083: F, t23085: F, t23087: F, t23098: F, t23751: F, t2972: F, t3804: F, t3806: F, t545: F, t642: F, t668: F, t670: F, t7843: F, t7852: F, t7866: F, t7868: F, t9829: F) -> (F, F) {
    let t27636 = t9833 * t1808;
    let t27641 = t7842 * t7848 * t9839;
    let t27649 = t9838 * t1808;
    let t27654 = t2970 * t7848 * t9834;
    let t27678 = t19 * t1815 * t3918;
    let t27684 = -7.0 / 144.0 * t7866 * t7868 * t27636 + t27641 / 24.0 + t7842 * t9829 * t7843 / 8.0 + t7842 * t2972 * t27636 / 16.0 - t7866 * t2972 * t27649 / 4.0 - t27654 / 72.0 - t2970 * t7852 * t9834 / 24.0 - t2970 * t2972 * t668 * t3804 * t545 / 24.0 - t1804 * t23751 * t1184 / 12.0 - 3.0 / 32.0 * t10087 * t642 - 3.0 / 32.0 * t10087 * t670 - 3.0 / 64.0 * t3806 * t1997 - 3.0 / 32.0 * t3806 * t1824 - 3.0 / 64.0 * t3806 * t1863 + t27678 / 96.0 - t23083 / 8.0 + t23085 / 24.0 + t23087 / 24.0 - t23098 / 32.0;
    (t27649, t27684)
}
