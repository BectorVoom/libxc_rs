//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 717/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk717<F: Float>(t5632: F, t664: F, t2006: F, t206: F, t2008: F, t1966: F, t188: F, t650: F, t5771: F, t621: F, t226: F, t5317: F) -> (F, F, F, F, F) {
    let t5782 = t5632 * t664;
    let t5785 = t2006 * t206;
    let t5786 = t2008 * t664;
    let t5787 = t5786 * t1966;
    let t5790 = t650 * t188;
    let t5791 = t5771 * t621;
    let t5793 = F::cast_from(18.0_f64) * t5790 * t5791;
    let t5794 = t226 * t5317;
    (t5782, t5785, t5787, t5793, t5794)
}
