//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 854/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk854<F: Float>(t376: F, t5706: F, t89: F, t492: F, t5617: F, t452: F, t488: F, t1307: F, t1820: F, t1825: F, t5644: F, t1871: F, t499: F, t5635: F, t110: F, t22970: F) -> (F, F, F, F, F, F, F, F) {
    let t23199 = t89 * t376 * t5706;
    let t23201 = t5617 * t492;
    let t23203 = t452 * t488 * t23201;
    let t23206 = t1307 * t1820;
    let t23208 = t452 * t488 * t23206;
    let t23212 = t452 * t1825 * t5644;
    let t23216 = t1871 * t499 * t5635;
    let t23220 = t1871 * t110 * t22970;
    (t23199, t23201, t23203, t23206, t23208, t23212, t23216, t23220)
}
