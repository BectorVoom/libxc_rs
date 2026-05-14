//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1137/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1137<F: Float>(t1403: F, t2399: F, t6839: F, t6837: F, t771: F, t6749: F, t96360: F, t1424: F, t4003: F, t28097: F, t761: F, t458: F, t6744: F, t6005: F, t27907: F, t681: F) -> (F, F, F, F, F, F, F, F) {
    let t109711 = t1403 * t2399 * t6839;
    let t109713 = t6837 * t771;
    let t109731 = t96360 * t6749;
    let t109735 = t1424 * t4003;
    let t109755 = t28097 * t761;
    let t109758 = t6744 * t458;
    let t109760 = t109758 * t6005 / 27.0;
    let t109767 = t1403 * t681 * t27907 / 9.0;
    (t109711, t109713, t109731, t109735, t109755, t109758, t109760, t109767)
}
