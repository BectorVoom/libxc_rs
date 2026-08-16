//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1344/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1344<F: Float>(t2272: F, t4148: F, t10743: F, t10759: F, t1359: F, t20990: F, t21043: F, t2246: F, t2276: F, t2285: F, t24911: F, t28932: F, t28973: F, t29028: F, t29031: F, t29033: F, t29036: F, t29038: F, t29040: F, t29042: F, t29044: F, t29046: F, t4167: F, t4170: F, t4197: F, t6673: F, t840: F, t848: F) -> F {
    let t29323 = t4148 * t2272;
    let t29337 = F::cast_from(1.0_f64) * t6673 * t4167 + F::cast_from(2.0_f64) * t2246 * t10743 - t28973 - t29028 + F::cast_from(0.32163958997385070134e2_f64) * t29323 * t2276 + F::cast_from(2.0_f64) * t24911 * t1359 - t29031 - t29033 - t29036 - t29038 - t29040 - t29042 + t29044 - t29046 + F::cast_from(0.11696447245269292414e1_f64) * t2285 * t10759 + F::cast_from(0.5848223622634646207e0_f64) * t840 * t28932 * t848 + F::cast_from(0.17315859105681463759e2_f64) * t20990 * t4197 + F::cast_from(0.32163958997385070134e2_f64) * t21043 * t4170;
    t29337
}
