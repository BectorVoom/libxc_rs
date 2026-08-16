//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2074;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2075;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta568(t2402: f64, t973: f64, t986: f64, t10213: f64, t135: f64, t41961: f64, t697: f64, t976: f64, t984: f64, t13797: f64, t10216: f64, t343: f64, t10383: f64, t964: f64, t10868: f64, t820: f64, t1015: f64, t10472: f64, t42559: f64, t204: f64, t376: f64, t1020: f64, t1023: f64, t248: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42903, t42972, t43002, t43052, t43053, t43069, t43070) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2074(t2402, t973, t986, t10213, t135, t41961, t697, t976, t984, t13797, t10216, t343);
        let (t43157, t43198, t43211, t43216, t43219) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2075(t10383, t964, t10868, t820, t1015, t10472, t42559, t204, t376, t1020, t1023, t248);
    (t42903, t42972, t43002, t43052, t43053, t43069, t43070, t43157, t43198, t43211, t43216, t43219)
}
