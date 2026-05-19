//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 785/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk785<F: Float>(t13745: F, t13759: F, t209: F, t12032: F, t921: F, t12860: F, t2355: F, t3718: F, t1382: F, t12866: F, t12870: F, t12873: F, t12877: F, t12880: F, t12883: F, t12884: F, t12889: F, t12893: F, t12896: F, t12898: F, t12902: F, t12906: F, t12909: F) -> (F, F, F, F, F, F, F, F) {
    let t13760 = t13745 + t13759;
    let t13761 = t13760 * t209;
    let t13762 = t12032 * t921;
    let t13763 = F::new(2.0) * t12860;
    let t13764 = t2355 * t3718;
    let t13765 = t3718 * t921;
    let t13766 = t1382 * t13765;
    let t13767 = F::new(2.0) * t13766;
    let t13772 = F::cast_from(0.11502877786176224903e2_f64) * t12866 + t12870 - t12873 + t12877 - t12880 - t12883 - F::cast_from(0.10725146985555128001e1_f64) * t12884 - t12889 + t12893 - t12896 + F::cast_from(0.71500979903700853338e0_f64) * t12898 + t12902 - F::cast_from(0.46011511144704899612e1_f64) * t12906 + t12909;
    (t13760, t13761, t13762, t13763, t13764, t13765, t13767, t13772)
}
