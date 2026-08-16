//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2215;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta586<F: Float>(t23514: F, t23545: F, t935: F, t915: F, t11387: F, t23466: F, t11385: F, t1642: F, t19049: F, t4719: F, t6223: F, t1699: F, t19153: F, t23448: F, t23450: F, t23455: F, t23459: F, t23461: F, t23463: F, t23465: F, t23469: F, t5023: F) -> (F, F, F, F, F, F, F, F) {
        let (t23546, t23547, t23549, t23550, t23552, t23554, t23556, t23560) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2215::<F>(t23514, t23545, t935, t915, t11387, t23466, t11385, t1642, t19049, t4719, t6223, t1699, t19153, t23448, t23450, t23455, t23459, t23461, t23463, t23465, t23469, t5023);
    (t23546, t23547, t23549, t23550, t23552, t23554, t23556, t23560)
}
