//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1252/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1252<F: Float>(t6889: F, t7700: F, t1985: F, t1807: F, t2006: F, t1811: F, t6916: F, t1799: F, t236: F) -> (F, F, F, F, F) {
    let t7701 = t6889 * t7700;
    let t7702 = t1985 * t7701;
    let t7704 = t1807 * t2006;
    let t7706 = t6916 * t1811;
    let t7708 = t236 * t1799;
    (t7701, t7702, t7704, t7706, t7708)
}
