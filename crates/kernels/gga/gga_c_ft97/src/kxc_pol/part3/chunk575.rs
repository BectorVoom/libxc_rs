//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 575/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk575<F: Float>(t5393: F, t871: F, t296: F, t319: F, t5299: F, t840: F, t1212: F, t1255: F, t992: F, t2875: F, t2874: F, t1248: F, t2882: F, t2881: F, t4969: F, t835: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5394 = t871 * t5393;
    let t5395 = t296 * t5394;
    let t5399 = t840 * t319 * t5299;
    let t5403 = t840 * t1255 * t1212;
    let t5408 = t992 * t1212;
    let t5409 = t2875 * t5408;
    let t5410 = t2874 * t5409;
    let t5413 = t992 * t1248;
    let t5414 = t2882 * t5413;
    let t5415 = t2881 * t5414;
    let t5419 = t835 * t319 * t4969;
    (t5394, t5395, t5399, t5403, t5408, t5409, t5410, t5414, t5415, t5419)
}
