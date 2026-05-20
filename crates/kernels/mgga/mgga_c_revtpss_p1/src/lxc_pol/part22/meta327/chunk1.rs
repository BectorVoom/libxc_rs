//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1779/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1779<F: Float>(t10858: F, t2726: F, t821: F, t235: F, t231: F, t2723: F) -> (F, F, F, F, F) {
    let t10859 = t10858 * t2726;
    let t10866 = t821 * t821;
    let t10867 = F::new(1.0) / t10866;
    let t10868 = t10867 * t235;
    let t10871 = t2723 * t231;
    (t10859, t10866, t10867, t10868, t10871)
}
