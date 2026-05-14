//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 830/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk830<F: Float>(t1376: F, t697: F, t1721: F, t424: F, t1707: F, t5709: F, t5727: F, t5736: F, t5739: F, t5754: F, t5761: F, t5766: F, t5889: F, t5891: F, t5895: F, t5897: F, t5898: F, t5901: F, t5903: F, t5907: F, t596: F) -> (F, F, F, F) {
    let t5908 = t1376 * t697;
    let t5910 = t424 * t1721;
    let t5912 = t424 * t1707;
    let t5914 = -t5889 - t5709 + t5727 - t5736 - t5739 - 3.0 * t5891 - t5895 + t5897 + 96.0 * t5898 - t5901 - t5754 - 0.2025780996e0 * t596 * t5903 + t5907 + 0.19518446340543131715e0 * t5908 + 0.19518446340543131715e0 * t5910 + 0.57791679765211885292e1 * t5912 + t5761 + t5766;
    (t5908, t5910, t5912, t5914)
}
