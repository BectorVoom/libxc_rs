//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta552 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1642;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1643;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta552<F: Float>(t23467: F, t52508: F, t6109: F, t11385: F, t2926: F, t23568: F, t4719: F, t23649: F, t18898: F, t64043: F, t981: F, t1699: F, t5023: F, t78478: F, t88004: F, t88007: F, t88012: F, t88016: F, t88023: F, t88026: F, t88028: F, t11506: F, t3014: F, t88008: F, t1610: F, t78097: F, t19056: F, t6142: F, t6145: F, t64336: F, t23547: F, t4590: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t88030, t88031, t88034, t88036, t88038, t88041, t88042) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1642::<F>(t23467, t52508, t6109, t11385, t2926, t23568, t4719, t23649, t18898, t64043, t981, t1699, t5023, t78478, t88004, t88007, t88012, t88016, t88023, t88026, t88028);
        let (t88046, t88048, t88050, t88052, t88054) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1643::<F>(t11506, t3014, t88008, t981, t1610, t78097, t19056, t6142, t6145, t64336, t23547, t4590);
    (t88030, t88031, t88034, t88036, t88038, t88041, t88042, t88046, t88048, t88050, t88052, t88054)
}
