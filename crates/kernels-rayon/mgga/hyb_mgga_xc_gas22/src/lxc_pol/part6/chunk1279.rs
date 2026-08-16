//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1279/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1279(t1808: f64, t9833: f64, t7842: f64, t7848: f64, t9839: f64, t9838: f64, t2970: f64, t9834: f64, t1815: f64, t19: f64, t3918: f64, t10087: f64, t1184: f64, t1804: f64, t1824: f64, t1863: f64, t1997: f64, t23083: f64, t23085: f64, t23087: f64, t23098: f64, t23751: f64, t2972: f64, t3804: f64, t3806: f64, t545: f64, t642: f64, t668: f64, t670: f64, t7843: f64, t7852: f64, t7866: f64, t7868: f64, t9829: f64) -> (f64, f64) {
    let t27636 = t9833 * t1808;
    let t27641 = t7842 * t7848 * t9839;
    let t27649 = t9838 * t1808;
    let t27654 = t2970 * t7848 * t9834;
    let t27678 = t19 * t1815 * t3918;
    let t27684 = -7.0_f64 / 144.0_f64 * t7866 * t7868 * t27636 + t27641 / 24.0_f64 + t7842 * t9829 * t7843 / 8.0_f64 + t7842 * t2972 * t27636 / 16.0_f64 - t7866 * t2972 * t27649 / 4.0_f64 - t27654 / 72.0_f64 - t2970 * t7852 * t9834 / 24.0_f64 - t2970 * t2972 * t668 * t3804 * t545 / 24.0_f64 - t1804 * t23751 * t1184 / 12.0_f64 - 3.0_f64 / 32.0_f64 * t10087 * t642 - 3.0_f64 / 32.0_f64 * t10087 * t670 - 3.0_f64 / 64.0_f64 * t3806 * t1997 - 3.0_f64 / 32.0_f64 * t3806 * t1824 - 3.0_f64 / 64.0_f64 * t3806 * t1863 + t27678 / 96.0_f64 - t23083 / 8.0_f64 + t23085 / 24.0_f64 + t23087 / 24.0_f64 - t23098 / 32.0_f64;
    (t27649, t27684)
}
