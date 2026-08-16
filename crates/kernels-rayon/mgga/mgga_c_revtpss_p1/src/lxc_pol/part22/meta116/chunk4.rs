//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 787/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk787(t213: f64, t234: f64, t2646: f64, t2724: f64, t2754: f64, t2760: f64, t2776: f64, t2780: f64, t2787: f64, t2791: f64, t2796: f64, t2802: f64, t2806: f64, t2810: f64, t2811: f64, t2815: f64, t820: f64, t837: f64, t879: f64) -> f64 {
    let t2828 = t2776 - t2780 + 0.10975748638225852664e-1_f64 * t2787 - 0.10975748638225852664e-1_f64 * t2791 + t2796 - 0.19514881078765566038e-1_f64 * t2802 + 0.19514881078765566038e-1_f64 * t2806 - t2810 + 0.13170898365871023197e1_f64 * t820 * t2811 * t2724 - 0.13170898365871023197e1_f64 * t820 * t2815 * t837 - 0.65854491829355115987e0_f64 * t820 * t879 * t2646 - 0.65854491829355115987e0_f64 * t820 * t879 * t2754 + 0.65854491829355115987e0_f64 * t213 * t234 * t2760;
    t2828
}
