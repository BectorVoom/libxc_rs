//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1668/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1668<F: Float>(t10726: F, t14868: F, t2661: F, t10868: F, t241: F, t820: F) -> (F, F, F) {
    let t14869 = t10726 * t14868;
    let t14871 = F::cast_from(0.28582678745379824648e-4_f64) * t2661 * t14869;
    let t14894 = t820 * t10868 * t241;
    (t14869, t14871, t14894)
}
