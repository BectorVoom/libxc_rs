//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1966/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1966(t2094: f64, t40611: f64, t12461: f64, t7216: f64, t1266: f64, t12734: f64, t1983: f64, t22574: f64, t2314: f64, t2323: f64, t23857: f64, t23933: f64, t24169: f64, t24433: f64, t24995: f64, t25988: f64, t26161: f64, t26163: f64, t26870: f64, t26902: f64, t26906: f64, t27147: f64, t27170: f64, t27171: f64, t27180: f64, t27188: f64, t32193: f64, t34711: f64, t4028: f64, t4034: f64, t510: f64, t5308: f64, t652: f64, t671: f64, t6876: f64, t7685: f64, t7806: f64, t7940: f64, t91655: f64, t91687: f64, t92128: f64) -> f64 {
    let t92169 = t2094 * t40611;
    let t92200 = t7216 * t12461;
    let t92210 = -6.0_f64 * t22574 * t32193 * t25988 - 4.0_f64 * t12734 * t7806 - 4.0_f64 * t2314 * t27180 - 6.0_f64 * t26161 * t92169 * t91687 - 6.0_f64 * t91655 * t24433 - 4.0_f64 * t4028 * t23933 - 4.0_f64 * t2314 * t27147 - 4.0_f64 * t652 * t26870 * t671 + 2.0_f64 * t7685 * t24169 - 2.0_f64 * t6876 * t26902 - 2.0_f64 * t652 * t510 * t92128 - 4.0_f64 * t27188 * t2323 - 4.0_f64 * t2314 * t27171 - 4.0_f64 * t4034 * t27171 - 4.0_f64 * t652 * t1266 * t27170 + 6.0_f64 * t6876 * t26906 + 4.0_f64 * t26161 * t92200 * t26163 + 12.0_f64 * t24995 * t34711 * t5308 + 2.0_f64 * t1983 * t7940 * t23857;
    t92210
}
