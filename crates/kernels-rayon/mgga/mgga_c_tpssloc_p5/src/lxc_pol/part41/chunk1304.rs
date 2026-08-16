//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1304/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1304(t2205: f64, t6483: f64, t1858: f64, t8283: f64, t30616: f64, t576: f64, t111302: f64, t111308: f64, t111310: f64, t111312: f64, t112051: f64, t1404: f64, t1852: f64, t20186: f64, t2206: f64, t3: f64, t30395: f64, t30582: f64, t5364: f64, t5381: f64, t580: f64, t8284: f64, t8299: f64) -> f64 {
    let t112083 = t2205 * t6483;
    let t112084 = t8283 * t1858;
    let t112087 = t576 * t30616;
    let t112090 = t112051 * t3 * t580 + t1404 * t30582 + 2.0_f64 * t1852 * t30395 + t20186 * t2206 + 2.0_f64 * t5364 * t8299 + 2.0_f64 * t5381 * t8284 + t111302 + t111308 + t111310 + t111312 + t112083 + 2.0_f64 * t112084 + t112087;
    t112090
}
