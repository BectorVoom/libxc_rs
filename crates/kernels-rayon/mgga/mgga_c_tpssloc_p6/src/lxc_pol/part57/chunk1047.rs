//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1047/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1047(t100911: f64, t115984: f64, t122811: f64, t127608: f64, t127627: f64, t127646: f64, t127647: f64, t127698: f64, t127701: f64, t127706: f64, t127708: f64, t127714: f64, t128976: f64, t128984: f64, t128988: f64, t1458: f64, t2039: f64, t23880: f64, t28951: f64, t29422: f64, t29425: f64, t5456: f64, t577: f64, t7010: f64) -> f64 {
    let t128989 = t127698 + t127701 + 0.135e2_f64 * t7010 * t28951 + t127608 + t127706 + t127708 + 54.0_f64 * t23880 * t29422 + 27.0_f64 * t23880 * t29425 + t127714 + 27.0_f64 * t115984 * t5456 + t127627 + 0.45e1_f64 * t128976 * t577 + 27.0_f64 * t122811 * t1458 + 27.0_f64 * t127647 * t2039 + t127646 + t128984 + 0.135e2_f64 * t100911 * t2039 + t128988;
    t128989
}
