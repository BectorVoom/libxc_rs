//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1344/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1344(t2272: f64, t4148: f64, t10743: f64, t10759: f64, t1359: f64, t20990: f64, t21043: f64, t2246: f64, t2276: f64, t2285: f64, t24911: f64, t28932: f64, t28973: f64, t29028: f64, t29031: f64, t29033: f64, t29036: f64, t29038: f64, t29040: f64, t29042: f64, t29044: f64, t29046: f64, t4167: f64, t4170: f64, t4197: f64, t6673: f64, t840: f64, t848: f64) -> f64 {
    let t29323 = t4148 * t2272;
    let t29337 = 1.0_f64 * t6673 * t4167 + 2.0_f64 * t2246 * t10743 - t28973 - t29028 + 0.32163958997385070134e2_f64 * t29323 * t2276 + 2.0_f64 * t24911 * t1359 - t29031 - t29033 - t29036 - t29038 - t29040 - t29042 + t29044 - t29046 + 0.11696447245269292414e1_f64 * t2285 * t10759 + 0.5848223622634646207e0_f64 * t840 * t28932 * t848 + 0.17315859105681463759e2_f64 * t20990 * t4197 + 0.32163958997385070134e2_f64 * t21043 * t4170;
    t29337
}
