//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 644/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk644<F: Float>(t1609: F, t77: F, t1593: F, t1608: F, t1615: F, t1630: F, t1711: F, t371: F, t407: F, t391: F, t625: F, t68: F, t72: F) -> (F, F, F, F, F) {
    let t8007 = t77 * t1609;
    let t8008 = t8007 * t1593;
    let t8009 = t1608 * t8008;
    let t8014 = t1615 * t1630;
    let t8015 = t1608 * t8014;
    let t8042 = t371 * t1711;
    let t8050 = t407 * t407;
    let t8051 = F::new(1.0) / t8050;
    let t8074 = t68 * t391 * t625 * t72;
    (t8009, t8015, t8042, t8051, t8074)
}
