//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 695/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk695<F: Float>(t12894: F, t4540: F, t12762: F, t1457: F, t1572: F, t12766: F, t12866: F, t12870: F, t12873: F, t12877: F, t12880: F, t12883: F, t12884: F, t12889: F, t12893: F) -> (F, F, F) {
    let t12896 = F::new(0.21450293971110256001e1) * t4540 * t12894;
    let t12897 = t1457 * t12762;
    let t12898 = t1572 * t12897;
    let t12900 = t1457 * t12766;
    let t12902 = F::new(0.71500979903700853338e0) * t1572 * t12900;
    let t12903 = F::new(0.23005755572352449806e2) * t12866 + t12870 - t12873 + t12877 - t12880 - t12883 - F::new(0.21450293971110256002e1) * t12884 - t12889 + t12893 - t12896 + F::new(0.14300195980740170668e1) * t12898 + t12902;
    (t12897, t12900, t12903)
}
