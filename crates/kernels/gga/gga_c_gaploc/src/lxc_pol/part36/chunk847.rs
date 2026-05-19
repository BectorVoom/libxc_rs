//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 847/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk847<F: Float>(t41906: F, t34478: F, t544: F, t9287: F, t12793: F, t1441: F, t40192: F, t40196: F, t41860: F, t41863: F, t41867: F, t41871: F, t41874: F, t41876: F, t41880: F, t41885: F, t41889: F, t41891: F, t41893: F, t41897: F, t41900: F, t41904: F, t41905: F, t536: F, t590: F) -> F {
    let t41907 = F::cast_from(0.15337170381568299871e1_f64) * t41906;
    let t41909 = t544 * t34478 * t9287;
    let t41911 = -F::cast_from(0.38342925953920749676e0_f64) * t40192 + F::cast_from(0.85206502119823888169e-1_f64) * t40196 + F::cast_from(0.35750489951850426669e0_f64) * t536 * t41860 + F::cast_from(0.23005755572352449806e2_f64) * t41863 + F::cast_from(0.23005755572352449806e2_f64) * t41867 + F::cast_from(0.23005755572352449806e2_f64) * t41871 + t41874 + F::cast_from(0.13803453343411469884e2_f64) * t41876 - F::cast_from(0.42900587942220512004e1_f64) * t41880 + t41885 - t41889 - F::cast_from(0.38342925953920749676e0_f64) * t41891 + t41893 + F::cast_from(0.51123901271894332902e0_f64) * t1441 * t12793 * t590 + F::cast_from(0.23833659967900284447e0_f64) * t41897 + F::cast_from(0.38342925953920749676e0_f64) * t41900 + t41904 - t41905 + t41907 + F::cast_from(0.29792074959875355558e-1_f64) * t41909;
    t41911
}
