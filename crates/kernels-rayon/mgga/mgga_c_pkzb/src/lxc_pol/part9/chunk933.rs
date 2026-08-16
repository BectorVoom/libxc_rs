//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 933/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk933(t1676: f64, t2706: f64, t192: f64, t2575: f64, t135: f64, t144: f64, t1535: f64, t1536: f64, t1692: f64, t2536: f64, t2714: f64, t2718: f64, t5077: f64, t5091: f64, t5130: f64, t5139: f64, t5141: f64, t560: f64, t568: f64, t637: f64, t639: f64, t6853: f64, t7010: f64, t7013: f64, t7015: f64, t7017: f64, t7018: f64, t7019: f64, t7020: f64, t7021: f64, t7022: f64, t7177: f64) -> (f64, f64, f64) {
    let t7181 = t2706 * t1676;
    let t7191 = t192 * t2575;
    let t7195 = 3.0_f64 * t135 * t560 * t6853 + t135 * t144 * t7177 * t639 + t7010 + t5077 - t7013 + t7015 - t7017 - t7018 - t7019 + t5091 - 2.0_f64 * t2536 * t7181 * t637 - t5130 - t7020 - t7021 + 3.0_f64 * t1535 * t2714 * t1692 + 6.0_f64 * t1535 * t1536 * t2575 + 12.0_f64 * t2718 * t7191 * t568 - t5139 - t5141 + t7022;
    (t7181, t7191, t7195)
}
