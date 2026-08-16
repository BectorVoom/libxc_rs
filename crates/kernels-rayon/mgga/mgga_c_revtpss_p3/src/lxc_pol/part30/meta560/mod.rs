//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta560 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2003;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2004;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta560(t92997: f64, t10671: f64, t7033: f64, t25255: f64, t2689: f64, t10680: f64, t1945: f64, t807: f64, t10690: f64, t9646: f64, t10674: f64, t7030: f64, t9789: f64, t2453: f64, t2783: f64, t64: f64, t10761: f64, t9784: f64, t2482: f64, t25260: f64, t27: f64, t10852: f64, t25266: f64, t2756: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92998, t93000, t93001, t93004, t93008, t93010, t93012) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2003(t92997, t10671, t7033, t25255, t2689, t10680, t1945, t807, t10690, t9646, t10674, t7030, t9789);
        let (t93013, t93015, t93016, t93021, t93026, t93028) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2004(t93012, t2453, t2783, t64, t10761, t7030, t9784, t2482, t25260, t27, t10852, t25266, t2756);
    (t92998, t93000, t93001, t93004, t93008, t93010, t93013, t93015, t93016, t93021, t93026, t93028)
}
