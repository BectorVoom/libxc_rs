//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 824/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk824(t11264: f64, t2268: f64, t6949: f64, t13277: f64, t6305: f64, t13268: f64, t13307: f64, t6313: f64, t42846: f64, t42849: f64, t39624: f64, t39626: f64, t39632: f64, t39637: f64, t39642: f64, t39646: f64, t39648: f64, t39650: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44572 = 0.34146007962811379518e0_f64 * t2268 * t11264 * t6949;
    let t44574 = 0.17073003981405689759e0_f64 * t6305 * t13277;
    let t44576 = 0.34146007962811379518e0_f64 * t6305 * t13268;
    let t44578 = 0.26558006193297739625e0_f64 * t6313 * t13307;
    let t44579 = 0.94850022118920498664e-2_f64 * t42846;
    let t44580 = 0.94850022118920498664e-2_f64 * t42849;
    let t44590 = (21.0_f64 / 256.0_f64 * t39624 + 357.0_f64 / 8192.0_f64 * t39626 - 189.0_f64 / 131072.0_f64 * t39632 + 189.0_f64 / 8388608.0_f64 * t39637 - 63.0_f64 / 8388608.0_f64 * t39642 + 63.0_f64 / 131072.0_f64 * t39646 - 119.0_f64 / 8192.0_f64 * t39648 - 7.0_f64 / 256.0_f64 * t39650) * t471;
    (t44572, t44574, t44576, t44578, t44579, t44580, t44590)
}
