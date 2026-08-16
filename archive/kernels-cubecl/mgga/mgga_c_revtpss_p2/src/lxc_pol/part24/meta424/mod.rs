//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta424 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1373;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1374;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta424<F: Float>(t12627: F, t1284: F, t3624: F, t3617: F, t675: F, t1263: F, t215: F, t1121: F, t13045: F, t221: F, t461: F, t462: F, t624: F, t12625: F, t458: F, t456: F, t225: F, t480: F, t43813: F, t126: F, t13099: F, t1224: F, t12268: F, t3566: F, t3781: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t44609, t44693, t44701, t44737, t44797) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1373::<F>(t12627, t1284, t3624, t3617, t675, t1263, t215, t1121, t13045, t221, t461, t462, t624);
        let (t44842, t44843, t44844, t44865, t44895, t44919, t44951) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1374::<F>(t12625, t458, t456, t225, t480, t43813, t126, t13099, t1224, t12268, t3566, t3781);
    (t44609, t44693, t44701, t44737, t44797, t44842, t44843, t44844, t44865, t44895, t44919, t44951)
}
