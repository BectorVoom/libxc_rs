//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 697/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk697<F: Float>(t103: F, t4436: F, t379: F, t8217: F, t16155: F, t3194: F, t8518: F, t16160: F, t8210: F, t3193: F, t432: F, t4431: F, t1903: F, t1902: F, t4545: F, t487: F) -> (F, F, F, F, F) {
    let t16228 = t103 * t4436;
    let t16229 = t16228 * t379;
    let t16230 = t8217 * t16229;
    let t16233 = t3194 * t16155;
    let t16234 = t8518 * t16233;
    let t16237 = t8210 * t16160;
    let t16238 = t3193 * t16237;
    let t16241 = t4431 * t432;
    let t16242 = t1903 * t16241;
    let t16243 = t1902 * t16242;
    let t16246 = t4545 * t487;
    (t16230, t16234, t16238, t16243, t16246)
}
