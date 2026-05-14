//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1181/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1181<F: Float>(t28835: F, t683: F, t43912: F, t6217: F, t28557: F, t28676: F, t213: F, t668: F, t505: F, t811: F, t108448: F, t22511: F, t28658: F, t7003: F, t820: F, t1196: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t111807 = t683 * t28835;
    let t111815 = t43912 * t6217;
    let t111830 = t28676 * t28557;
    let t111831 = t213 * t668;
    let t111832 = t505 * t811;
    let t111834 = t108448 * t111831 * t111832;
    let t111837 = t28658 * t22511;
    let t111838 = t7003 * t111837;
    let t111839 = t505 * t820;
    let t111841 = t108448 * t111831 * t111839;
    let t111844 = t1196 * t668;
    (t111807, t111815, t111830, t111832, t111834, t111837, t111838, t111839, t111841, t111844)
}
