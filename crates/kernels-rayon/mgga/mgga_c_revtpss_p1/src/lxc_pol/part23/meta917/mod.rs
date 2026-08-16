//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta917 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2957;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2958;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta917(t23452: f64, t974: f64, t981: f64, t15258: f64, t19467: f64, t4708: f64, t6226: f64, t19049: f64, t4734: f64, t1699: f64, t5023: f64, t68207: f64, t77657: f64, t78417: f64, t78422: f64, t78426: f64, t78428: f64, t78432: f64, t23696: f64, t3022: f64, t15537: f64, t6206: f64, t4725: f64, t23451: f64, t41235: f64, t41238: f64, t972: f64, t23446: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t78435, t78438, t78441, t78443, t78444) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2957(t23452, t974, t981, t15258, t19467, t4708, t6226, t19049, t4734, t1699, t5023, t68207, t77657, t78417, t78422, t78426, t78428, t78432);
        let (t78446, t78449, t78451, t78456, t78458) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2958(t23696, t3022, t15537, t6206, t981, t19049, t4725, t23451, t41235, t41238, t972, t23446);
    (t78435, t78438, t78441, t78443, t78444, t78446, t78449, t78451, t78456, t78458)
}
