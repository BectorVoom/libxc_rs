//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 679/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk679<F: Float>(t1954: F, t8361: F, t8362: F, t8328: F, t8330: F, t8332: F, t8335: F, t8338: F, t8342: F, t8344: F, t8348: F, t8354: F, t8359: F, t132: F, t2878: F, t2881: F) -> (F, F, F) {
    let t8364 = t8361 * t8362 * t1954;
    let t8366 = 0.10427789137624512459e-2 * t8328 - 0.3475929712541504153e-2 * t8330 - 0.3475929712541504153e-2 * t8332 - 0.21724560703384400956e-4 * t8335 - 0.12672660410307567224e-4 * t8338 - 0.21724560703384400956e-4 * t8342 - 0.43449121406768801912e-4 * t8344 - 0.12672660410307567224e-4 * t8348 - 0.75106634031756181751e-6 * t8354 + 0.772525378612349298e-5 * t8359 + 0.42242201367691890748e-6 * t8364;
    let t8368 = t132 * t2878;
    let t8369 = t8368 * t2881;
    (t8364, t8366, t8369)
}
