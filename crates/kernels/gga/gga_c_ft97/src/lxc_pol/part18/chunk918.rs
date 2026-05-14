//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 918/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk918<F: Float>(t1391: F, t1986: F, t2185: F, t2075: F, t574: F, t558: F, t5975: F, t5956: F, t9276: F, t144: F, t1384: F, t605: F, t2142: F, t5947: F, t167: F, t23884: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23953 = t2185 * t1391 * t1986;
    let t23957 = t574 * t1391 * t2075;
    let t23961 = t574 * t5975 * t558;
    let t23964 = t9276 * t5956;
    let t23965 = t144 * t23964;
    let t23968 = t1384 * t2075;
    let t23970 = t574 * t605 * t23968;
    let t23974 = t574 * t2142 * t5947;
    let t23978 = t574 * t167 * t23884;
    (t23953, t23957, t23961, t23964, t23965, t23968, t23970, t23974, t23978)
}
