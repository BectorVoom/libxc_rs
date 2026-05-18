//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1138/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1138<F: Float>(t43946: F, t43950: F, t43955: F, t43959: F, t43961: F, t43964: F, t43968: F, t43972: F, t43975: F, t43977: F, t43980: F, t43983: F) -> F {
    let t47458 = F::new(0.11502877786176224903e2) * t43946 + F::new(0.11502877786176224903e2) * t43950 - t43955 - t43959 - t43961 - F::new(0.46011511144704899612e1) * t43964 - F::new(0.46011511144704899612e1) * t43968 - t43972 - t43975 + t43977 - t43980 - t43983;
    t47458
}
