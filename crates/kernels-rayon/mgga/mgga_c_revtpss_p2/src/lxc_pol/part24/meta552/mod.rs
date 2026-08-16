//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta552 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1642;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1643;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta552(t23467: f64, t52508: f64, t6109: f64, t11385: f64, t2926: f64, t23568: f64, t4719: f64, t23649: f64, t18898: f64, t64043: f64, t981: f64, t1699: f64, t5023: f64, t78478: f64, t88004: f64, t88007: f64, t88012: f64, t88016: f64, t88023: f64, t88026: f64, t88028: f64, t11506: f64, t3014: f64, t88008: f64, t1610: f64, t78097: f64, t19056: f64, t6142: f64, t6145: f64, t64336: f64, t23547: f64, t4590: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t88030, t88031, t88034, t88036, t88038, t88041, t88042) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1642(t23467, t52508, t6109, t11385, t2926, t23568, t4719, t23649, t18898, t64043, t981, t1699, t5023, t78478, t88004, t88007, t88012, t88016, t88023, t88026, t88028);
        let (t88046, t88048, t88050, t88052, t88054) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1643(t11506, t3014, t88008, t981, t1610, t78097, t19056, t6142, t6145, t64336, t23547, t4590);
    (t88030, t88031, t88034, t88036, t88038, t88041, t88042, t88046, t88048, t88050, t88052, t88054)
}
