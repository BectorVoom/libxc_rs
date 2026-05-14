//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 658/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk658<F: Float>(t2083: F, t3651: F, t1175: F, t1180: F, t5684: F, t2089: F, t827: F, t3661: F, t5671: F, t26: F, t1186: F, t5676: F, t22: F, t3118: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5730 = t3651 * t2083;
    let t5731 = t5730 * t1175;
    let t5733 = t1180 * t5684;
    let t5736 = t827 * t2089;
    let t5738 = t3661 * t5671;
    let t5739 = t26 * t5738;
    let t5741 = t1186 * t5676;
    let t5742 = t26 * t5741;
    let t5744 = t22 * t3118;
    (t5730, t5731, t5733, t5736, t5738, t5739, t5741, t5742, t5744)
}
