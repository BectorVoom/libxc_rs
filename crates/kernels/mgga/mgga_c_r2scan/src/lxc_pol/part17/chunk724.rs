//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 724/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk724<F: Float>(t5717: F, t61: F, t1721: F, t424: F, t1707: F, t124: F, t717: F, t722: F, t1762: F, t1732: F, t1771: F, t230: F, t4889: F) -> (F, F, F, F, F, F) {
    let t5907 = F::new(0.11407595979765752406e3) * t61 * t5717;
    let t5910 = t424 * t1721;
    let t5912 = t424 * t1707;
    let t5916 = t124 * t717;
    let t5917 = t5916 * t722;
    let t5919 = F::new(0.64212977516902094772e0) * t1762 * t5917;
    let t5920 = t1771 * t1732;
    let t5923 = F::new(120.0) * t4889 * t230;
    (t5907, t5910, t5912, t5919, t5920, t5923)
}
