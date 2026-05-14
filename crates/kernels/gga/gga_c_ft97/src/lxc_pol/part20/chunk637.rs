//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 637/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk637<F: Float>(t13739: F, t13746: F, t13753: F, t13728: F, t13732: F, t13736: F, t13743: F, t13750: F, t13759: F, t9872: F, t9876: F, t13780: F, t13794: F, t13764: F, t13768: F, t13772: F, t13775: F, t13778: F, t13783: F, t13786: F, t13789: F, t13792: F) -> (F, F) {
    let t13981 = 4.0 / 9.0 * t13739;
    let t13983 = 4.0 / 3.0 * t13746;
    let t13984 = 2.0 / 3.0 * t13753;
    let t13986 = 4.0 * t13728 - 22.0 / 9.0 * t13732 + 2.0 / 3.0 * t13736 - t13981 + 2.0 * t13743 - t13983 - t13750 + t13984 - t9872 - t9876 - 4.0 / 3.0 * t13759;
    let t13993 = 2.0 / 9.0 * t13780;
    let t13998 = 4.0 / 27.0 * t13794;
    let t13999 = -t13764 / 4.0 + 3.0 / 8.0 * t13768 - t13772 / 2.0 + t13775 / 3.0 + 2.0 / 9.0 * t13778 - t13993 + 2.0 / 3.0 * t13783 - 4.0 / 3.0 * t13786 + 2.0 / 3.0 * t13789 - 8.0 / 3.0 * t13792 + t13998;
    (t13986, t13999)
}
