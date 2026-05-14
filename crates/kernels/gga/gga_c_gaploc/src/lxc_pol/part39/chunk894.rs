//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 894/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk894<F: Float>(t23000: F, t33308: F, t9889: F, t43007: F, t739: F, t1991: F, t590: F, t43107: F, t1890: F, t1966: F, t10948: F, t11016: F, t13012: F, t2087: F, t4614: F, t3267: F, t8634: F) -> (F, F, F, F, F, F, F, F) {
    let t43832 = t23000 * t33308 * t9889;
    let t43833 = 0.11502877786176224903e1 * t43832;
    let t43834 = t739 * t43007;
    let t43836 = t1991 * t43834 * t590;
    let t43838 = t739 * t43107;
    let t43841 = 0.1022478025437886658e1 * t1991 * t43838 * t590;
    let t43842 = t1890 * t43007;
    let t43844 = t1966 * t43842 * t590;
    let t43849 = 0.25561950635947166451e1 * t1966 * t1890 * t43107 * t590;
    let t43854 = t10948 * t11016;
    let t43858 = 0.92023022289409799224e1 * t2087 * t4614 * t13012;
    let t43861 = 0.35750489951850426669e0 * t3267 * t8634;
    (t43833, t43836, t43841, t43844, t43849, t43854, t43858, t43861)
}
