//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta198 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk909;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk910;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta198(t1843: f64, t670: f64, t2616: f64, t2524: f64, t1534: f64, t72: f64, t757: f64, t1469: f64, t750: f64, t706: f64, t190: f64, t4186: f64, t1531: f64, t705: f64, t707: f64, t2498: f64, t2518: f64, t2522: f64, t2526: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t2610: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4297, t4300, t4301, t4302, t4304, t4305, t4307, t4308) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk909(t1843, t670, t2616, t2524, t1534, t72, t757, t1469, t750, t706, t190, t4186);
        let (t4310, t4311, t4313, t4314) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk910(t4308, t706, t1531, t705, t707, t2498, t2518, t2522, t2526, t2562, t2569, t2579, t2587, t2610, t4300, t4301, t4304, t4307);
    (t4297, t4300, t4301, t4302, t4304, t4305, t4307, t4308, t4310, t4311, t4313, t4314)
}
