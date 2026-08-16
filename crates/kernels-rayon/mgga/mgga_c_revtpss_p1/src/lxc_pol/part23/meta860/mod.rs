//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta860 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta860(t21342: f64, t460: f64, t1276: f64, t6587: f64, t487: f64, t70208: f64, t1269: f64, t20849: f64, t1770: f64, t5412: f64, t3555: f64, t6695: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t72959, t73051, t73055, t73137, t73187, t73205) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2750(t21342, t460, t1276, t6587, t487, t70208, t1269, t20849, t1770, t5412, t3555, t6695);
    (t72959, t73051, t73055, t73137, t73187, t73205)
}
