//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 688/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk688<F: Float>(t11906: F, t3183: F, t4589: F, t487: F, t379: F, t1909: F, t11467: F, t11593: F, t16024: F, t16027: F, t16031: F, t16036: F, t16040: F, t16044: F, t16049: F, t16054: F, t16057: F, t16062: F, t16067: F, t16070: F, t1901: F, t446: F) -> (F,) {
    let t16073 = t11906 * t3183;
    let t16076 = t487 * t4589;
    let t16077 = t16076 * t379;
    let t16078 = t1909 * t16077;
    let t16081 = 2.0 / 9.0 * t1901 * t16024 + 4.0 / 9.0 * t1901 * t16027 - 4.0 / 27.0 * t1901 * t16031 - 2.0 / 9.0 * t1901 * t16036 + 2.0 / 9.0 * t446 * t16040 + 2.0 / 3.0 * t446 * t16044 + 2.0 / 3.0 * t446 * t16049 + t11467 + t1901 * t16054 / 9.0 + 2.0 / 9.0 * t1901 * t16057 + 2.0 / 9.0 * t1901 * t16062 + 4.0 / 9.0 * t11593 * t16067 + 2.0 / 9.0 * t1901 * t16070 + 2.0 / 9.0 * t1901 * t16073 + t1901 * t16078 / 9.0;
    (t16081,)
}
