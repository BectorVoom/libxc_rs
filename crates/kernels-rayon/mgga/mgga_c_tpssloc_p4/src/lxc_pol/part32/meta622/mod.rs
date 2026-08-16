//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2028;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2029;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta622(t11647: f64, t2141: f64, t24684: f64, t27634: f64, t461: f64, t607: f64, t1009: f64, t7324: f64, t24658: f64, t27635: f64, t3540: f64, t7334: f64, t11832: f64, t2127: f64, t10401: f64, t24739: f64, t3610: f64, t3624: f64, t24740: f64, t3604: f64, t11791: f64, t7345: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86191, t86234, t86261, t86264, t86275) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2028(t11647, t2141, t24684, t27634, t461, t607, t1009, t7324, t24658, t27635, t3540, t7334);
        let (t86278, t86292, t86324, t86327, t86330, t86348) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2029(t11832, t2127, t24684, t7324, t10401, t24739, t3610, t3624, t24740, t3604, t11791, t7345);
    (t86191, t86234, t86261, t86264, t86275, t86278, t86292, t86324, t86327, t86330, t86348)
}
