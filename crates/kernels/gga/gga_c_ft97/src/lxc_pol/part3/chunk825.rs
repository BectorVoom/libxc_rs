//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 825/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk825<F: Float>(t2007: F, t4466: F, t15776: F, t528: F, t120: F, t15656: F, t72: F, t4687: F, t8959: F, t422: F, t4441: F, t8966: F) -> (F, F, F, F, F) {
    let t16842 = t2007 * t4466;
    let t16845 = t528 * t15776;
    let t16848 = t15656 * t120;
    let t16849 = t72 * t16848;
    let t16853 = F::new(0.8854768453090786061e-3) * t8959 * t4687;
    let t16854 = t422 * t4441;
    let t16855 = t16854 * t8966;
    (t16842, t16845, t16849, t16853, t16855)
}
