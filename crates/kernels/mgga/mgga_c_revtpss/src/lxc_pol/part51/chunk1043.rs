//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1043/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1043<F: Float>(t2037: F, t7956: F, t1913: F, t8617: F, t34015: F, t571: F, t2042: F, t28246: F, t1916: F, t32369: F, t2040: F, t28277: F, t28264: F, t572: F, t7741: F, t5795: F, t8611: F) -> (F, F, F, F, F, F, F, F) {
    let t127428 = t2037 * t7956;
    let t127434 = t1913 * t8617;
    let t127437 = t571 * t34015;
    let t127439 = t28246 * t2042;
    let t127442 = 12.0 * t1916 * t32369;
    let t127443 = t2040 * t28277;
    let t127447 = 12.0 * t572 * t28264 * t7741;
    let t127449 = 6.0 * t5795 * t8611;
    (t127428, t127434, t127437, t127439, t127442, t127443, t127447, t127449)
}
