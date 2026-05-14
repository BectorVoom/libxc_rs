//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 847/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk847<F: Float>(t12767: F, t6313: F, t2268: F, t2756: F, t3152: F, t39866: F, t39869: F, t39893: F, t39895: F, t39897: F, t39899: F, t39901: F, t39904: F, t1063: F, t7974: F, t41809: F, t426: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42863 = 0.7588001769513639893e-1 * t6313 * t12767;
    let t42866 = 0.28455006635676149599e-1 * t2268 * t3152 * t2756;
    let t42867 = 0.47425011059460249332e-2 * t39866;
    let t42868 = 0.94850022118920498664e-2 * t39869;
    let t42869 = 0.71137516589190373998e-2 * t39893;
    let t42870 = 0.23712505529730124666e-2 * t39895;
    let t42871 = 0.31616674039640166221e-2 * t39897;
    let t42872 = 0.23712505529730124666e-2 * t39899;
    let t42873 = 0.94850022118920498664e-2 * t39901;
    let t42874 = 0.71137516589190373998e-2 * t39904;
    let t42877 = 0.28455006635676149599e-1 * t1063 * t3152 * t7974;
    let t42878 = t41809 * t426;
    (t42863, t42866, t42867, t42868, t42869, t42870, t42871, t42872, t42873, t42874, t42877, t42878)
}
