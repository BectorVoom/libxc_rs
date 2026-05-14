//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1054/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1054<F: Float>(t24543: F, t27754: F, t27758: F, t27764: F, t27772: F, t96925: F, t24526: F, t3875: F, t24432: F, t6118: F, t14103: F, t6135: F, t13892: F, t27805: F, t3886: F, t97198: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t107996 = t24543 * t27754;
    let t107997 = 2.0 / 9.0 * t107996;
    let t107998 = t24543 * t27758;
    let t107999 = 2.0 / 9.0 * t107998;
    let t108000 = t24543 * t27764;
    let t108001 = 2.0 / 27.0 * t108000;
    let t108002 = t96925 * t27772;
    let t108003 = t108002 / 18.0;
    let t108004 = t24526 * t3875;
    let t108006 = t6118 * t24432 * t108004;
    let t108008 = t6135 * t14103;
    let t108010 = t6118 * t24432 * t108008;
    let t108012 = t6135 * t13892;
    let t108014 = t27805 * t24432 * t108012;
    let t108016 = t97198 * t3886;
    (t107996, t107997, t107998, t107999, t108000, t108001, t108002, t108003, t108004, t108006, t108008, t108010, t108012, t108014, t108016)
}
