//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1115/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1115<F: Float>(t25899: F, t94634: F, t25885: F, t25889: F, t25921: F, t26084: F, t4078: F, t4131: F, t543: F, t7274: F, t7295: F, t7296: F, t7301: F, t7304: F, t94591: F, t94593: F, t94598: F, t94602: F, t94605: F, t94608: F, t94610: F, t94613: F, t94616: F, t94628: F) -> (F,) {
    let t94635 = t25899 * t94634;
    let t94637 = 0.13709901006661042888e-1 * t94591 + 0.51405703062096148812e-1 * t94593 - 0.86736281882051994623e-1 * t94598 + t94602 - 0.43368140941025997312e-1 * t94605 - t94608 + 0.13010442282307799193e1 * t94610 * t7304 + 0.77108554593144223218e-1 * t94613 + 0.34697458558045176417e-2 * t94616 + 0.39512695097613069591e1 * t26084 * t4078 + 0.26020884564615598386e1 * t25921 * t25885 + 0.52041769129231196772e1 * t25921 * t25889 + 0.26020884564615598386e1 * t7295 * t7296 * t7274 * t4131 + 0.13010442282307799193e1 * t7295 * t7301 * t94628 * t543 - 0.51405703062096148812e-1 * t94635;
    (t94637,)
}
