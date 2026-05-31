//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1211/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1211<F: Float>(t10556: F, t1535: F, t1536: F, t16825: F, t16946: F, t16950: F, t20377: F, t2718: F, t29146: F, t29149: F, t29150: F, t29151: F, t6758: F, t8758: F, t8779: F, t9112: F) -> F {
    let t29744 = F::cast_from(3.0_f64) * t10556 * t1535 * t1536 + F::cast_from(18.0_f64) * t1535 * t8758 * t8779 + F::cast_from(18.0_f64) * t2718 * t6758 * t9112 + t16825 + t16946 + t16950 - t20377 - t29146 - t29149 - t29150 + t29151;
    t29744
}
