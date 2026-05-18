//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1193/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1193<F: Float>(t20378: F, t16822: F, t16825: F, t16946: F, t16950: F, t20365: F, t20373: F, t20377: F, t29139: F, t29140: F, t29141: F, t29142: F, t29143: F, t29145: F, t29146: F, t29149: F, t29150: F) -> (F, F) {
    let t29151 = F::new(180.0) * t20378;
    let t29152 = -t16822 - t20365 - t29139 - t29140 - t29141 + t29142 + t20373 - t29143 + t29145 + t16825 - t29146 - t20377 + t16946 + t16950 - t29149 - t29150 + t29151;
    (t29151, t29152)
}
