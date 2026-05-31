//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1213/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1213<F: Float>(t3409: F, t6249: F, t3382: F, t5869: F, t1106: F, t1181: F, t1889: F, t3361: F, t1180: F, t16996: F, t17000: F, t17002: F, t17016: F, t17018: F, t17020: F, t17029: F, t17752: F, t4261: F, t4711: F, t5697: F, t5862: F) -> F {
    let t22135 = t3409 * t6249;
    let t22144 = t3382 * t5869;
    let t22155 = t3361 * t1181 * t1889 * t1106;
    let t22157 = F::cast_from(0.40015750243531754508e-2_f64) * t22135 - F::cast_from(0.42874018118069736972e-3_f64) * t1180 * t1181 * t5862 * t4711 - F::cast_from(0.68598428988911579156e-2_f64) * t16996 + F::cast_from(0.51448821741683684367e-2_f64) * t17000 + F::cast_from(0.34299214494455789578e-1_f64) * t17002 + F::cast_from(0.85748036236139473944e-3_f64) * t22144 + F::cast_from(0.13605355082800796532e0_f64) * t17016 + F::cast_from(0.40015750243531754508e-2_f64) * t17018 - F::cast_from(0.80031500487063509015e-1_f64) * t17020 + F::cast_from(0.17149607247227894789e-1_f64) * t17029 - t4261 * t17752 * t5697 / F::cast_from(6.0_f64) + F::cast_from(0.34299214494455789578e-2_f64) * t22155;
    t22157
}
