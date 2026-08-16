//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1018/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1018(t41276: f64, t8761: f64, t1635: f64, t2084: f64, t8746: f64, t1624: f64, t8764: f64, t5181: f64, t649: f64, t36119: f64, t1627: f64, t7599: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41294 = t8761 * t41276;
    let t41296 = t2084 * t1635;
    let t41297 = t8746 * t41296;
    let t41298 = 0.12122071846331262991e0_f64 * t41297;
    let t41299 = t8761 * t41296;
    let t41300 = 0.45158592333657918156e-2_f64 * t41299;
    let t41301 = t2084 * t1624;
    let t41302 = t8764 * t41301;
    let t41303 = 0.36366215538993788972e-1_f64 * t41302;
    let t41304 = t649 * t5181;
    let t41305 = t36119 * t41304;
    let t41307 = t2084 * t1627;
    let t41308 = t7599 * t41307;
    (t41294, t41298, t41300, t41301, t41303, t41304, t41305, t41307, t41308)
}
