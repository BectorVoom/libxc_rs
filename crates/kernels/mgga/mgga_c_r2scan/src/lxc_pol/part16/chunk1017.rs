//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1017/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1017<F: Float>(t10657: F, t11566: F, t11570: F, t11580: F, t11585: F, t12382: F, t12386: F, t12388: F, t12394: F, t12398: F, t12425: F, t12432: F, t12578: F) -> F {
    let t12580 = -t12382 - F::new(0.30487649791575028314e-3) * t11566 + F::new(0.43368970657079495312e-4) * t11570 - t12386 + t12388 + t12394 + t12398 - t10657 + F::new(0.19211284388664477842e-2) * t11580 + F::new(0.72042316457491791906e-3) * t11585 + t12425 + t12432 + t12578;
    t12580
}
