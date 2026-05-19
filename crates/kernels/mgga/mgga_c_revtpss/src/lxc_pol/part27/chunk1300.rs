//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1300/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1300<F: Float>(t5: F, t96748: F, t96779: F, t96803: F, t96830: F, t117: F, t10259: F, t2371: F, t27060: F, t29432: F, t670: F, t7586: F, t94956: F, t94958: F, t94960: F, t94962: F, t94964: F, t94966: F, t94968: F, t94970: F, t94972: F, t94993: F, t96706: F, t96709: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t96833 = piecewise3::<F>(t8, F::new(0.0), t96748 + t96779 + t96803 + t96830);
    let t96834 = t96833 * t117;
    let t96835 = F::new(2.0) * t10259 * t7586 + F::new(6.0) * t2371 * t27060 + F::new(6.0) * t2371 * t29432 + F::new(6.0) * t670 * t96706 + t94956 + t94958 + t94960 + t94962 + t94964 + t94966 + t94968 + t94970 + t94972 + t94993 + F::new(6.0) * t96709 + t96834;
    (t96834, t96835)
}
