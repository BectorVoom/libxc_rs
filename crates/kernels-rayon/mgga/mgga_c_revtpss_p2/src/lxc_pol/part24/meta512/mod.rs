//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta512 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1528;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1529;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta512(t11922: f64, t11927: f64, t23838: f64, t23998: f64, t3115: f64, t23916: f64, t3091: f64, t43131: f64, t15618: f64, t19785: f64, t23820: f64, t3153: f64, t15707: f64, t19920: f64, t23891: f64, t3127: f64, t3172: f64, t19697: f64, t4820: f64, t1032: f64, t1040: f64, t23959: f64, t19658: f64, t4879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t78802, t78805, t78855, t78863, t78873) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1528(t11922, t11927, t23838, t23998, t3115, t23916, t3091, t43131, t15618, t19785, t23820, t3153);
        let (t78910, t78915, t78986, t79038, t79071) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1529(t15707, t19920, t23891, t3127, t3172, t19697, t4820, t1032, t1040, t23959, t19658, t4879);
    (t78802, t78805, t78855, t78863, t78873, t78910, t78915, t78986, t79038, t79071)
}
