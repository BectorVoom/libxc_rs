//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 742/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk742<F: Float>(t2478: F, t3358: F, t6576: F, t3177: F, t8272: F, t9267: F, t40208: F, t12953: F, t4781: F, t34478: F, t544: F, t9287: F, t12793: F, t1441: F, t40192: F, t40196: F, t41860: F, t41863: F, t41867: F, t41871: F, t41874: F, t41876: F, t41880: F, t41885: F, t41889: F, t41891: F, t41893: F, t41897: F, t536: F, t590: F) -> (F,) {
    let t41900 = t6576 * t3358 * t2478;
    let t41903 = t9267 * t8272 * t3177;
    let t41904 = 0.19171462976960374838e1 * t41903;
    let t41905 = 0.10352590007558602413e2 * t40208;
    let t41906 = t4781 * t12953;
    let t41907 = 0.15337170381568299871e1 * t41906;
    let t41909 = t544 * t34478 * t9287;
    let t41911 = -0.38342925953920749676e0 * t40192 + 0.85206502119823888169e-1 * t40196 + 0.35750489951850426669e0 * t536 * t41860 + 0.23005755572352449806e2 * t41863 + 0.23005755572352449806e2 * t41867 + 0.23005755572352449806e2 * t41871 + t41874 + 0.13803453343411469884e2 * t41876 - 0.42900587942220512004e1 * t41880 + t41885 - t41889 - 0.38342925953920749676e0 * t41891 + t41893 + 0.51123901271894332902e0 * t1441 * t12793 * t590 + 0.23833659967900284447e0 * t41897 + 0.38342925953920749676e0 * t41900 + t41904 - t41905 + t41907 + 0.29792074959875355558e-1 * t41909;
    (t41911,)
}
