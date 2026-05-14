//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1173/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1173<F: Float>(t11335: F, t930: F, t37594: F, t6426: F, t1608: F, t5566: F, t65750: F, t22522: F, t22572: F, t25760: F, t1631: F, t929: F, t22642: F, t22819: F, t25793: F, t384: F, t45540: F) -> (F, F, F, F, F, F, F) {
    let t100834 = t930 * t11335;
    let t100838 = t6426 * t37594;
    let t100843 = t1608 * t5566 * t65750;
    let t100848 = t22522 * t22572 * t25760;
    let t100850 = t1631 * t929;
    let t100880 = 0.60548059007656442388e-3 * t22819 * t22642 * t25793;
    let t100881 = t45540 * t384;
    (t100834, t100838, t100843, t100848, t100850, t100880, t100881)
}
