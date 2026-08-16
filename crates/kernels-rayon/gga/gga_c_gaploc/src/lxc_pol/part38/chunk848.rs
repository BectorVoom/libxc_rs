//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 848/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk848(t10215: f64, t2787: f64, t11264: f64, t2268: f64, t6949: f64, t13277: f64, t6305: f64, t13268: f64, t13307: f64, t6313: f64, t42846: f64, t42849: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44564 = t2787 * t10215;
    let t44572 = 0.34146007962811379518e0_f64 * t2268 * t11264 * t6949;
    let t44574 = 0.17073003981405689759e0_f64 * t6305 * t13277;
    let t44576 = 0.34146007962811379518e0_f64 * t6305 * t13268;
    let t44578 = 0.26558006193297739625e0_f64 * t6313 * t13307;
    let t44579 = 0.94850022118920498664e-2_f64 * t42846;
    let t44580 = 0.94850022118920498664e-2_f64 * t42849;
    (t44564, t44572, t44574, t44576, t44578, t44579, t44580)
}
