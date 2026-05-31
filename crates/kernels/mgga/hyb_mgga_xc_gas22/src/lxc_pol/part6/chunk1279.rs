//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1279/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1279<F: Float>(t1808: F, t9833: F, t7842: F, t7848: F, t9839: F, t9838: F, t2970: F, t9834: F, t1815: F, t19: F, t3918: F, t10087: F, t1184: F, t1804: F, t1824: F, t1863: F, t1997: F, t23083: F, t23085: F, t23087: F, t23098: F, t23751: F, t2972: F, t3804: F, t3806: F, t545: F, t642: F, t668: F, t670: F, t7843: F, t7852: F, t7866: F, t7868: F, t9829: F) -> (F, F) {
    let t27636 = t9833 * t1808;
    let t27641 = t7842 * t7848 * t9839;
    let t27649 = t9838 * t1808;
    let t27654 = t2970 * t7848 * t9834;
    let t27678 = t19 * t1815 * t3918;
    let t27684 = -F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t7866 * t7868 * t27636 + t27641 / F::cast_from(24.0_f64) + t7842 * t9829 * t7843 / F::cast_from(8.0_f64) + t7842 * t2972 * t27636 / F::cast_from(16.0_f64) - t7866 * t2972 * t27649 / F::cast_from(4.0_f64) - t27654 / F::cast_from(72.0_f64) - t2970 * t7852 * t9834 / F::cast_from(24.0_f64) - t2970 * t2972 * t668 * t3804 * t545 / F::cast_from(24.0_f64) - t1804 * t23751 * t1184 / F::cast_from(12.0_f64) - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t10087 * t642 - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t10087 * t670 - F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t3806 * t1997 - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t3806 * t1824 - F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t3806 * t1863 + t27678 / F::cast_from(96.0_f64) - t23083 / F::cast_from(8.0_f64) + t23085 / F::cast_from(24.0_f64) + t23087 / F::cast_from(24.0_f64) - t23098 / F::cast_from(32.0_f64);
    (t27649, t27684)
}
