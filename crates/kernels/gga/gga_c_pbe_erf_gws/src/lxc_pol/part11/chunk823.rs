//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 823/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk823<F: Float>(t13140: F, t825: F, t1114: F, t8520: F, t12332: F, t12333: F, t12334: F, t13070: F, t13071: F, t4341: F, t4349: F, t4499: F, t4503: F, t4506: F, t4513: F, t4539: F, t4542: F) -> (F, F, F) {
    let t13141 = t13140 * t825;
    let t13142 = t1114 * t13141;
    let t13148 = F::new(0.18981728898494541632e1) * t8520;
    let t13149 = -t13070 - t12332 + t13071 + t4341 - t4349 - t4499 + t4503 - t4506 - t4513 + t4539 + t4542 + t12333 - t13148 + t12334;
    (t13141, t13142, t13149)
}
