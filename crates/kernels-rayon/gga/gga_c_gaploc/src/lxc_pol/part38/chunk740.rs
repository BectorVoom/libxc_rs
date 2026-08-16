//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 740/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk740(t10012: f64, t8669: f64, t2925: f64, t321: f64, t1: f64, t22623: f64, t8502: f64, t2021: f64, t8774: f64, t10007: f64, t197: f64, t2754: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24549 = t10012 * t8669;
    let t24884 = t321 * t2925;
    let t24885 = t24884 * t1;
    let t25070 = t22623 * t8502;
    let t25198 = t2021 * t8774;
    let t25359 = t10007 * t8669;
    let t25760 = t197 * t2754;
    (t24549, t24884, t24885, t25070, t25198, t25359, t25760)
}
