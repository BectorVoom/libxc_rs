//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1126/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1126<F: Float>(t23660: F, t95053: F, t23642: F, t23649: F, t23646: F, t23663: F, t1368: F, t1771: F, t5902: F, t23609: F, t23611: F, t376: F, t23637: F, t5890: F, t23632: F, t1637: F, t5921: F, t89: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t95054 = t95053 * t23660;
    let t95078 = t23649 * t23642;
    let t95087 = t23649 * t23646;
    let t95094 = t23649 * t23663;
    let t95099 = t1368 * t1771;
    let t95100 = t95099 * t5902;
    let t95107 = t23609 * t376 * t23611;
    let t95151 = t5890 * t376 * t23637;
    let t95154 = t5890 * t376 * t23632;
    let t95177 = t89 * t1637 * t5921;
    (t95054, t95078, t95087, t95094, t95099, t95100, t95107, t95151, t95154, t95177)
}
