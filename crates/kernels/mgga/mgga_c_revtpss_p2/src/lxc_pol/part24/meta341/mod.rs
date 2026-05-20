//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1191;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1192;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta341<F: Float>(t23535: F, t916: F, t923: F, t1600: F, t6113: F, t11354: F, t11358: F, t11334: F, t11338: F, t18919: F, t18924: F, t18934: F, t19002: F, t19004: F, t19009: F, t23521: F, t23523: F, t23514: F, t935: F, t915: F, t11387: F, t23466: F, t11385: F, t1642: F, t19049: F, t4719: F, t6223: F, t1699: F, t19153: F, t23448: F, t23450: F, t23455: F, t23459: F, t23461: F, t23463: F, t23465: F, t23469: F, t5023: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23536, t23538, t23541, t23543, t23545) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1191::<F>(t23535, t916, t923, t1600, t6113, t11354, t11358, t11334, t11338, t18919, t18924, t18934, t19002, t19004, t19009, t23521, t23523);
        let (t23546, t23547, t23549, t23550, t23552, t23554, t23556, t23560) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1192::<F>(t23514, t23545, t935, t915, t11387, t23466, t11385, t1642, t19049, t4719, t6223, t1699, t19153, t23448, t23450, t23455, t23459, t23461, t23463, t23465, t23469, t5023);
    (t23536, t23538, t23541, t23543, t23546, t23547, t23549, t23550, t23552, t23554, t23556, t23560)
}
