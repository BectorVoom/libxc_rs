//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1157/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1157<F: Float>(t121184: F, t8477: F, t32673: F, t686: F, t72: F, t32710: F, t32705: F, t121211: F, t32685: F, t689: F, t121131: F, t121365: F) -> (F, F, F, F, F, F) {
    let t122455 = t8477 * t121184;
    let t122463 = t32673 * t72 * t686;
    let t122464 = t32710 * t122463;
    let t122466 = t32705 * t122463;
    let t122468 = F::new(0.47023883532522246276e-4) * t121211;
    let t122474 = t32685 * t689;
    let t122475 = t121131 * t122474;
    let t122477 = t121365 * t122474;
    (t122455, t122464, t122466, t122468, t122475, t122477)
}
