//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 551/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk551<F: Float>(t200: F, t4977: F, t680: F, t2379: F, t4960: F, t1113: F, t203: F, t237: F, t1127: F, t3767: F, t207: F, t215: F, t1101: F, t206: F, t214: F, t52: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4978 = t4977 * t200;
    let t4979 = t680 * t4978;
    let t4982 = t2379 * t4960;
    let t4985 = t1113 * t1113;
    let t4986 = t203 * t4985;
    let t4987 = t4986 * t237;
    let t4991 = t3767 * t1127;
    let t4995 = 1.0 / t207 / t215;
    let t4999 = t215 * t1101;
    let t5001 = 1.0 / t206 / t4999;
    let t5003 = t52 * t214 * t5001;
    (t4978, t4979, t4982, t4985, t4986, t4987, t4991, t4995, t5001, t5003)
}
