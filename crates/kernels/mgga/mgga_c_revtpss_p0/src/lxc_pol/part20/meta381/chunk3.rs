//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1386/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1386<F: Float>(t10785: F, t10943: F, t124: F, t14791: F, t2730: F, t40240: F, t40655: F, t40748: F, t40750: F, t40753: F, t40759: F, t40761: F, t40765: F, t40771: F, t40782: F, t40784: F, t40789: F, t40792: F, t40801: F, t40804: F, t40810: F, t4362: F, t4364: F, t4366: F, t800: F) -> F {
    let t40811 = -F::cast_from(0.48018900292238105408e-1_f64) * t40748 + F::cast_from(0.12004725073059526352e-1_f64) * t40750 - F::cast_from(0.27107389498472794074e-4_f64) * t40753 - t40759 - F::cast_from(0.27107389498472794074e-4_f64) * t40761 + F::cast_from(0.32528867398167352889e-3_f64) * t40765 + t40771 + F::new(3.0) / F::new(16.0) * t2730 * t800 * t124 * t40240 - F::cast_from(0.20579528696673473747e-1_f64) * t4362 * t14791 * t4366 * t40655 + F::cast_from(0.6046824481244798459e0_f64) * t40782 + F::cast_from(0.68026775414003982664e0_f64) * t40784 - F::cast_from(0.17149607247227894789e-2_f64) * t40789 + F::new(35.0) / F::new(12.0) * t40792 + F::cast_from(0.77173232612525526552e-2_f64) * t4362 * t4364 * t10785 * t10943 + F::cast_from(0.54214778996945588149e-4_f64) * t40801 - F::cast_from(0.30492001685571196936e-3_f64) * t40804 + t40810;
    t40811
}
