//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 802/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk802<F: Float>(t24890: F, t6274: F, t24898: F, t6365: F, t15369: F, t29055: F, t6374: F, t15460: F, t312: F, t7584: F, t684: F, t10492: F) -> (F, F, F, F, F, F, F, F) {
    let t34070 = t24890 * t6274;
    let t34073 = t24898 * t6365;
    let t34074 = t15369 * t34073;
    let t34077 = t29055 * t6374;
    let t34078 = t15460 * t34077;
    let t34081 = t312 * t7584;
    let t34082 = t34081 * t684;
    let t34083 = t10492 * t34082;
    (t34070, t34073, t34074, t34077, t34078, t34081, t34082, t34083)
}
