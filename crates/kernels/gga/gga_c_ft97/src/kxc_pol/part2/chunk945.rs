//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 945/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk945<F: Float>(t14660: F, t2665: F, t446: F, t1934: F, t4051: F, t13352: F, t2857: F, t1091: F, t2682: F, t10248: F, t13346: F, t835: F) -> (F, F, F, F, F, F, F) {
    let t14661 = t2665 * t14660;
    let t14662 = t446 * t14661;
    let t14664 = t4051 * t1934;
    let t14665 = t2665 * t14664;
    let t14666 = t446 * t14665;
    let t14668 = t2857 * t13352;
    let t14669 = t446 * t14668;
    let t14671 = t1091 * t2682;
    let t14672 = t10248 * t14671;
    let t14673 = t446 * t14672;
    let t14675 = t835 * t13346;
    (t14662, t14664, t14666, t14669, t14671, t14673, t14675)
}
