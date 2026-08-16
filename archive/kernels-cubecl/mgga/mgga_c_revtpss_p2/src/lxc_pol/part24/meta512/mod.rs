//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta512 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1528;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1529;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta512<F: Float>(t11922: F, t11927: F, t23838: F, t23998: F, t3115: F, t23916: F, t3091: F, t43131: F, t15618: F, t19785: F, t23820: F, t3153: F, t15707: F, t19920: F, t23891: F, t3127: F, t3172: F, t19697: F, t4820: F, t1032: F, t1040: F, t23959: F, t19658: F, t4879: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t78802, t78805, t78855, t78863, t78873) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1528::<F>(t11922, t11927, t23838, t23998, t3115, t23916, t3091, t43131, t15618, t19785, t23820, t3153);
        let (t78910, t78915, t78986, t79038, t79071) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1529::<F>(t15707, t19920, t23891, t3127, t3172, t19697, t4820, t1032, t1040, t23959, t19658, t4879);
    (t78802, t78805, t78855, t78863, t78873, t78910, t78915, t78986, t79038, t79071)
}
