//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta917 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2957;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2958;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta917<F: Float>(t23452: F, t974: F, t981: F, t15258: F, t19467: F, t4708: F, t6226: F, t19049: F, t4734: F, t1699: F, t5023: F, t68207: F, t77657: F, t78417: F, t78422: F, t78426: F, t78428: F, t78432: F, t23696: F, t3022: F, t15537: F, t6206: F, t4725: F, t23451: F, t41235: F, t41238: F, t972: F, t23446: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t78435, t78438, t78441, t78443, t78444) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2957::<F>(t23452, t974, t981, t15258, t19467, t4708, t6226, t19049, t4734, t1699, t5023, t68207, t77657, t78417, t78422, t78426, t78428, t78432);
        let (t78446, t78449, t78451, t78456, t78458) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2958::<F>(t23696, t3022, t15537, t6206, t981, t19049, t4725, t23451, t41235, t41238, t972, t23446);
    (t78435, t78438, t78441, t78443, t78444, t78446, t78449, t78451, t78456, t78458)
}
