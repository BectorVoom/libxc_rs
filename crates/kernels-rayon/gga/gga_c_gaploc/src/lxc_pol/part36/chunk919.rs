//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 919/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk919(t12767: f64, t6313: f64, t2268: f64, t2756: f64, t3152: f64, t39866: f64, t39869: f64, t39893: f64, t39895: f64, t39897: f64, t39899: f64, t39901: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42863 = 0.7588001769513639893e-1_f64 * t6313 * t12767;
    let t42866 = 0.28455006635676149599e-1_f64 * t2268 * t3152 * t2756;
    let t42867 = 0.47425011059460249332e-2_f64 * t39866;
    let t42868 = 0.94850022118920498664e-2_f64 * t39869;
    let t42869 = 0.71137516589190373998e-2_f64 * t39893;
    let t42870 = 0.23712505529730124666e-2_f64 * t39895;
    let t42871 = 0.31616674039640166221e-2_f64 * t39897;
    let t42872 = 0.23712505529730124666e-2_f64 * t39899;
    let t42873 = 0.94850022118920498664e-2_f64 * t39901;
    (t42863, t42866, t42867, t42868, t42869, t42870, t42871, t42872, t42873)
}
