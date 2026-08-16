//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1316/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1316<F: Float>(t498: F, t5713: F, t16078: F, t16069: F, t5701: F, t12125: F, t12129: F, t12131: F, t12145: F, t12149: F, t1368: F, t16874: F, t16878: F, t16881: F, t16886: F, t16889: F) -> F {
    let t16892 = t5713 * t498;
    let t16893 = t16892 * t16078;
    let t16896 = t5701 * t16069;
    let t16899 = -t12125 / F::cast_from(288.0_f64) + t12131 / F::cast_from(216.0_f64) + t12145 / F::cast_from(144.0_f64) + t12129 - t12149 / F::cast_from(432.0_f64) - t1368 * t16874 / F::cast_from(36.0_f64) + t1368 * t16878 / F::cast_from(144.0_f64) + t1368 * t16881 / F::cast_from(48.0_f64) + t1368 * t16886 / F::cast_from(72.0_f64) - t1368 * t16889 / F::cast_from(144.0_f64) + t1368 * t16893 / F::cast_from(36.0_f64) + t1368 * t16896 / F::cast_from(216.0_f64);
    t16899
}
