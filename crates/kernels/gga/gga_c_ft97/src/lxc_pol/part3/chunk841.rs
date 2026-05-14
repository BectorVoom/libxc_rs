//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 841/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk841<F: Float>(t1217: F, t18986: F, t4134: F, t5206: F, t2648: F, t5304: F, t1091: F, t4162: F, t10248: F, t446: F, t1212: F, t505: F, t3699: F, t2665: F, t3690: F, t10409: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18987 = t18986 * t1217;
    let t18989 = t5206 * t4134;
    let t18992 = t2648 * t5304;
    let t18997 = t1091 * t4162;
    let t18998 = t10248 * t18997;
    let t18999 = t446 * t18998;
    let t19001 = t1212 * t505;
    let t19002 = t3699 * t19001;
    let t19003 = t2665 * t19002;
    let t19004 = t446 * t19003;
    let t19006 = t3690 * t19001;
    let t19007 = t10409 * t19006;
    (t18987, t18989, t18992, t18997, t18999, t19002, t19004, t19006, t19007)
}
