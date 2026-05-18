//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1338/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1338<F: Float>(t2030: F, t47567: F, t1444: F, t4057: F, t26069: F, t94806: F, t1426: F, t94609: F, t7063: F, t7286: F, t7289: F, t94810: F) -> (F, F, F, F, F, F) {
    let t94867 = F::new(0.81814717454467823679e-4) * t47567 * t2030;
    let t94868 = t4057 * t1444;
    let t94876 = t26069 * t94806;
    let t94878 = t94609 * t1426;
    let t94879 = t7063 * t94878;
    let t94880 = t94879 * t7286;
    let t94882 = t7289 * t94810;
    (t94867, t94868, t94876, t94878, t94880, t94882)
}
