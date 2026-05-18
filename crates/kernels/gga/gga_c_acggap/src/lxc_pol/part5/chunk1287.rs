//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1287/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1287<F: Float>(t1861: F, t3228: F, t1008: F, t5956: F, t13084: F, t5737: F, t1165: F, t1173: F, t1180: F, t1181: F, t14243: F, t14245: F, t1426: F, t14260: F, t1532: F, t1552: F, t175: F, t18460: F, t20433: F, t21677: F, t22048: F, t3169: F, t3196: F, t418: F, t5852: F) -> F {
    let t23864 = t3228 * t1861;
    let t23866 = t1008 * t5956;
    let t23872 = t13084 * t5737;
    let t23886 = F::new(0.60023625365297631762e-2) * t18460 - F::new(0.40015750243531754508e-1) * t14243 + F::new(0.12862205435420921092e-2) * t14245 + t14260 + F::new(0.85748036236139473944e-2) * t418 * t1426 * t175 * t21677 - F::new(0.34299214494455789578e-2) * t23864 - F::new(0.68598428988911579156e-2) * t23866 - F::new(0.85748036236139473944e-3) * t1180 * t1181 * t1532 * t22048 + F::new(0.80031500487063509016e-1) * t23872 + F::new(0.85748036236139473944e-3) * t1180 * t1165 * t1552 * t20433 + F::new(0.17149607247227894789e-2) * t1173 * t1165 * t5852 * t3196 - F::new(0.85748036236139473944e-3) * t1180 * t1181 * t5852 * t3169;
    t23886
}
