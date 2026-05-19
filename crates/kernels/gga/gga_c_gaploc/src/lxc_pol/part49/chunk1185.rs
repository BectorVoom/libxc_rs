//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1185/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1185<F: Float>(t12065: F, t2441: F, t38759: F, t895: F, t10348: F, t12054: F, t41885: F, t41889: F, t41891: F, t41893: F, t41897: F, t41900: F, t41904: F, t41905: F, t41907: F) -> F {
    let t47937 = t2441 * t12065;
    let t47939 = t895 * t38759;
    let t47941 = t12054 * t10348;
    let t47946 = F::cast_from(0.35750489951850426669e0_f64) * t47937 + F::cast_from(0.35750489951850426669e0_f64) * t47939 - F::cast_from(0.7150097990370085334e0_f64) * t47941 + t41885 - t41889 - F::cast_from(0.19171462976960374838e0_f64) * t41891 + t41893 + F::cast_from(0.11916829983950142223e0_f64) * t41897 + F::cast_from(0.19171462976960374838e0_f64) * t41900 + t41904 - t41905 + t41907;
    t47946
}
