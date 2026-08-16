//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1386/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1386(t10785: f64, t10943: f64, t124: f64, t14791: f64, t2730: f64, t40240: f64, t40655: f64, t40748: f64, t40750: f64, t40753: f64, t40759: f64, t40761: f64, t40765: f64, t40771: f64, t40782: f64, t40784: f64, t40789: f64, t40792: f64, t40801: f64, t40804: f64, t40810: f64, t4362: f64, t4364: f64, t4366: f64, t800: f64) -> f64 {
    let t40811 = -0.48018900292238105408e-1_f64 * t40748 + 0.12004725073059526352e-1_f64 * t40750 - 0.27107389498472794074e-4_f64 * t40753 - t40759 - 0.27107389498472794074e-4_f64 * t40761 + 0.32528867398167352889e-3_f64 * t40765 + t40771 + 3.0_f64 / 16.0_f64 * t2730 * t800 * t124 * t40240 - 0.20579528696673473747e-1_f64 * t4362 * t14791 * t4366 * t40655 + 0.6046824481244798459e0_f64 * t40782 + 0.68026775414003982664e0_f64 * t40784 - 0.17149607247227894789e-2_f64 * t40789 + 35.0_f64 / 12.0_f64 * t40792 + 0.77173232612525526552e-2_f64 * t4362 * t4364 * t10785 * t10943 + 0.54214778996945588149e-4_f64 * t40801 - 0.30492001685571196936e-3_f64 * t40804 + t40810;
    t40811
}
