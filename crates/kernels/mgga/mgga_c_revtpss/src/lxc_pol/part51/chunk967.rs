//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 967/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk967<F: Float>(t32255: F, t32257: F, t2022: F, t7274: F, t8707: F, t25875: F, t8590: F, t1381: F, t31805: F, t555: F) -> (F, F, F, F, F) {
    let t32258 = t32255 * t32257;
    let t32262 = t8707 * t2022 * t7274;
    let t32265 = t25875 * t8590;
    let t32266 = t32265 * t1381;
    let t32267 = F::new(0.1859366460452550541e-4) * t32266;
    let t32268 = t31805 * t555;
    (t32258, t32262, t32265, t32267, t32268)
}
