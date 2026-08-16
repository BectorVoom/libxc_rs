//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta424 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1373;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1374;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta424(t12627: f64, t1284: f64, t3624: f64, t3617: f64, t675: f64, t1263: f64, t215: f64, t1121: f64, t13045: f64, t221: f64, t461: f64, t462: f64, t624: f64, t12625: f64, t458: f64, t456: f64, t225: f64, t480: f64, t43813: f64, t126: f64, t13099: f64, t1224: f64, t12268: f64, t3566: f64, t3781: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44609, t44693, t44701, t44737, t44797) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1373(t12627, t1284, t3624, t3617, t675, t1263, t215, t1121, t13045, t221, t461, t462, t624);
        let (t44842, t44843, t44844, t44865, t44895, t44919, t44951) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1374(t12625, t458, t456, t225, t480, t43813, t126, t13099, t1224, t12268, t3566, t3781);
    (t44609, t44693, t44701, t44737, t44797, t44842, t44843, t44844, t44865, t44895, t44919, t44951)
}
