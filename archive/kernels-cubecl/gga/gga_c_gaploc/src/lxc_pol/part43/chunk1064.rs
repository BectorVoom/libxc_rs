//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1064/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1064<F: Float>(t43938: F, t43955: F, t43959: F, t43961: F, t43972: F, t43975: F, t43977: F, t43980: F, t43983: F, t43986: F, t43989: F, t43993: F, t43994: F, t44002: F, t47442: F, t47445: F, t47448: F, t47450: F, t47462: F, t47463: F) -> F {
    let t51156 = -t43938 - t47442 - F::cast_from(0.71500979903700853338e0_f64) * t47445 + F::cast_from(0.13803453343411469884e2_f64) * t47448 - F::cast_from(0.59584149919750711116e-1_f64) * t47450 - t43955 - t43959 - t43961 - t43972 - t43975 + t43977 - t43980 - t43983 + t43986 - t43989 + t47462 - t43993 - t43994 - t47463 + t44002;
    t51156
}
