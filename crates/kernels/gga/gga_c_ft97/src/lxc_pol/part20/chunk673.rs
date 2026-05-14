//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 673/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk673<F: Float>(t14668: F, t446: F, t1091: F, t2682: F, t10248: F, t13346: F, t835: F, t2876: F, t4051: F) -> (F, F, F, F, F) {
    let t14669 = t446 * t14668;
    let t14671 = t1091 * t2682;
    let t14672 = t10248 * t14671;
    let t14673 = t446 * t14672;
    let t14675 = t835 * t13346;
    let t14676 = t446 * t14675;
    let t14678 = t4051 * t2876;
    (t14669, t14671, t14673, t14676, t14678)
}
