//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2303;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta586<F: Float>(t1501: F, t670: F, t14613: F, t162: F, t1553: F, t73: F, t2723: F, t4423: F, t1544: F, t890: F, t1651: F, t3268: F) -> (F, F, F, F, F, F) {
        let (t18227, t18259, t18592, t18632, t18875, t19428) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2303::<F>(t1501, t670, t14613, t162, t1553, t73, t2723, t4423, t1544, t890, t1651, t3268);
    (t18227, t18259, t18592, t18632, t18875, t19428)
}
