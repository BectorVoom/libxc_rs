//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 574/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk574<F: Float>(t6889: F, t6907: F, t1985: F, t1323: F, t2006: F, t1887: F, t534: F, t6546: F) -> (F, F, F, F) {
    let t6908 = t6889 * t6907;
    let t6909 = t1985 * t6908;
    let t6911 = t1323 * t2006;
    let t6914 = t6546 * t534 * t1887;
    (t6908, t6909, t6911, t6914)
}
