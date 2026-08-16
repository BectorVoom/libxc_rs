//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 998/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk998(t218: f64, t219: f64, t7984: f64, t3026: f64, t824: f64, t334: f64, t7945: f64, t1174: f64, t6149: f64, t2204: f64, t6165: f64, t6175: f64, t6177: f64, t7970: f64, t7973: f64, t7975: f64, t7980: f64, t7983: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7986 = t218 * t219 * t7984;
    let t7988 = t824 * t3026;
    let t7990 = t218 * t219 * t7988;
    let t7992 = t334 * t7945;
    let t7994 = t218 * t219 * t7992;
    let t7996 = t6149 * t1174;
    let t7997 = t7996 * t2204;
    let t7999 = t6165 * t1174;
    let t8000 = t7999 * t2204;
    let t8002 = -0.9494625e0_f64 * t7970 + 0.3071625e0_f64 * t7973 + 0.15358125e0_f64 * t7975 - t6175 + 0.54771111111111111111e0_f64 * t6177 - t7980 - t7983 + 0.24647e0_f64 * t7986 + 0.49294e0_f64 * t7990 + 0.24647e0_f64 * t7994 + 0.142419375e1_f64 * t7997 - 0.76790625e-1_f64 * t8000;
    (t7986, t7988, t7990, t7992, t7994, t7996, t7997, t7999, t8000, t8002)
}
